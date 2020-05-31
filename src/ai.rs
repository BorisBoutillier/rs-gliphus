use crate::{
    components::{Actuator, Cardinal, Movable, Player, Position},
    gui::{draw_ui, MainMenuSelection},
    map,
    player::{try_actuate, try_move_player},
    turn_history::{TurnState, TurnsHistory},
    RunState,
};
use bracket_lib::prelude::*;
use legion::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AiAction {
    ExitTo(i32, i32),
    PushAt(i32, i32, Cardinal),
    ActivateAt(i32, i32),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AiSubAction {
    Move(Cardinal),
    Actuate,
}
impl AiSubAction {
    /// Play this SubAction in the World, and return if it was successful
    /// Successful meaning that the state of the world changed.
    fn play(&self, ecs: &mut World, rsrc: &mut Resources) -> bool {
        let actions = match &self {
            AiSubAction::Move(cardinal) => try_move_player(*cardinal, ecs, rsrc),
            AiSubAction::Actuate => try_actuate(ecs, rsrc),
        };
        if actions.len() > 0 {
            let mut turn_history = rsrc.get_mut::<TurnsHistory>().unwrap();
            turn_history.play_turn(ecs, actions);
            true
        } else {
            false
        }
    }
}

pub struct AI {
    pub show: bool,
    pub dead_ends: u64,
    pub solutions: u64,
    pub duplicates: u64,
    pub paused: bool,
    pub finished: bool,
    history: AiHistory,
    sub_actions: Vec<AiSubAction>,
    sub_actions_success: bool,
}
//fn a_star_search<T>(start: T, end: T, map: &dyn BaseMap) -> NavigationPath
impl AI {
    pub fn new() -> AI {
        AI {
            show: true,
            dead_ends: 0,
            solutions: 0,
            duplicates: 0,
            paused: true,
            finished: false,
            history: AiHistory {
                possibilities: vec![],
            },
            sub_actions: vec![],
            sub_actions_success: true,
        }
    }
    pub fn draw_state(&self, rsrc: &Resources, ctx: &mut BTerm) {
        draw_ui(rsrc, ctx);
        ctx.print(20, 1, format!("Dead-ends : {}", self.dead_ends));
        ctx.print(20, 2, format!("Duplicates: {}", self.duplicates));
        ctx.print(20, 3, format!("Solutions : {}", self.solutions));
        ctx.print(20, 4, format!("Min Energy: {}", "None"));
        if self.paused {
            ctx.print_color_centered(6, RGB::named(RED1), RGB::named(LIGHT_GRAY), "Paused!");
        }
    }
    pub fn play_next_turn(
        &mut self,
        ecs: &mut World,
        rsrc: &mut Resources,
        ctx: &mut BTerm,
    ) -> RunState {
        if !self.paused {
            if self.sub_actions.is_empty() {
                let mut turn_history = rsrc.get_mut::<TurnsHistory>().unwrap();
                let cur_step = turn_history.steps;
                if self.sub_actions_success {
                    self.history
                        .possibilities
                        .push((cur_step, self.find_possible_actions(ecs, rsrc)));
                } else {
                    if turn_history.state == TurnState::Running {
                        self.dead_ends += 1;
                    }
                    while !self.history.possibilities.is_empty()
                        && self.history.possibilities.last().unwrap().1.is_empty()
                    {
                        self.history.possibilities.pop();
                    }
                    if self.history.possibilities.is_empty() {
                        turn_history.undo(cur_step, ecs);
                        self.paused = true;
                        self.finished = true;
                        return RunState::GameDraw;
                    } else {
                        let undo_steps = cur_step - self.history.possibilities.last().unwrap().0;
                        turn_history.undo(undo_steps, ecs);
                    }
                }
                let (_tested_action, sub_actions) = self
                    .history
                    .possibilities
                    .last_mut()
                    .unwrap()
                    .1
                    .pop()
                    .unwrap();
                //println!(
                //    "Depth {}. Doing {:?}",
                //    self.history.possibilities.len(),
                //    tested_action,
                //);
                self.sub_actions = sub_actions;
            }
            if !self.sub_actions.is_empty() {
                let action = self.sub_actions.remove(0);
                self.sub_actions_success = action.play(ecs, rsrc);
            }
        }
        self.get_next_runstate(ctx)
    }
    fn get_next_runstate(&mut self, ctx: &mut BTerm) -> RunState {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => {
                    self.paused = !self.paused;
                }
                VirtualKeyCode::S => {
                    self.show = !self.show;
                }
                VirtualKeyCode::Escape => {
                    return RunState::MainMenu {
                        menu_selection: MainMenuSelection::Continue,
                    }
                }
                _ => {}
            }
        }
        if self.paused {
            RunState::GameDraw
        } else {
            RunState::GameTurn
        }
    }
    pub fn end_turn(&mut self, rsrc: &Resources) -> RunState {
        if self.paused {
            return RunState::GameAwaitingInput;
        }
        if self.finished {
            let level = rsrc.get::<map::Map>().unwrap().level;
            *self = AI::new();
            return RunState::LoadLevel(level + 1);
        }
        let curstate = rsrc.get::<TurnsHistory>().unwrap().state;
        match curstate {
            TurnState::PlayerDead => {
                panic!("Unexpected dying of player in an AI run");
            }
            TurnState::PlayerAtExit => {
                self.solutions += 1;
                self.sub_actions_success = false;
                RunState::GameAwaitingInput
            }
            TurnState::Running => RunState::GameAwaitingInput,
        }
    }
    pub fn find_possible_actions(
        &self,
        ecs: &World,
        rsrc: &Resources,
    ) -> Vec<(AiAction, Vec<AiSubAction>)> {
        let mut actions = vec![];
        let map = rsrc.get::<map::Map>().unwrap();
        let query = <(Read<Position>,)>::query().filter(tag::<Player>());
        for (player_pos,) in query.iter(&ecs) {
            // Go to Exit
            for (x, y) in map.get_exits().into_iter() {
                if let Some(directions) = map.try_go_to((player_pos.x, player_pos.y), (x, y)) {
                    let mut sub_actions = vec![];

                    for &direction in directions.iter() {
                        sub_actions.push(AiSubAction::Move(direction));
                    }
                    actions.push((AiAction::ExitTo(x, y), sub_actions));
                }
            }
            // Go to Movables and push
            let query2 = <(Read<Position>,)>::query().filter(tag::<Movable>());
            for (movable_pos,) in query2.iter(&ecs) {
                for direction in &[Cardinal::N, Cardinal::S, Cardinal::W, Cardinal::E] {
                    let (dx, dy) = direction.get_delta_xy();
                    if let Some(directions) = map.try_go_to(
                        (player_pos.x, player_pos.y),
                        (movable_pos.x + dx, movable_pos.y + dy),
                    ) {
                        let mut sub_actions = vec![];
                        // Move actions
                        for &direction in directions.iter() {
                            sub_actions.push(AiSubAction::Move(direction));
                        }
                        //Push action
                        sub_actions.push(AiSubAction::Move(direction.inv()));
                        actions.push((
                            AiAction::PushAt(movable_pos.x, movable_pos.y, direction.inv()),
                            sub_actions,
                        ));
                    }
                }
            }
            // Go to Activable and Activate
            let query2 = <(Read<Position>,)>::query().filter(tag::<Actuator>());
            for (activable_pos,) in query2.iter(&ecs) {
                for direction in &[Cardinal::N, Cardinal::S, Cardinal::W, Cardinal::E] {
                    let (dx, dy) = direction.get_delta_xy();
                    if let Some(directions) = map.try_go_to(
                        (player_pos.x, player_pos.y),
                        (activable_pos.x + dx, activable_pos.y + dy),
                    ) {
                        let mut sub_actions = vec![];
                        // Move actions
                        for &direction in directions.iter() {
                            sub_actions.push(AiSubAction::Move(direction));
                        }
                        //Actuate action
                        sub_actions.push(AiSubAction::Actuate);
                        actions.push((
                            AiAction::ActivateAt(activable_pos.x, activable_pos.y),
                            sub_actions,
                        ));
                    }
                }
            }
        }
        actions
    }
}

struct AiHistory {
    possibilities: Vec<(i32, Vec<(AiAction, Vec<AiSubAction>)>)>,
}
