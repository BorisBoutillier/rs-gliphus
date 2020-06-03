use crate::{
    ai_cache::AiStatesCache,
    components::{Actuator, Cardinal, Movable, Player, Position},
    gui::{draw_ui, MainMenuSelection},
    map,
    player::{try_actuate, try_move_player, try_teleport_player},
    turn_history::{TurnState, TurnsHistory},
    RunState, TERM_WIDTH,
};
use bracket_lib::prelude::*;
use legion::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;

#[derive(Clone, Debug, PartialEq)]
pub enum AiAction {
    ExitTo(i32, i32),
    PushAt(i32, i32, Cardinal),
    ActivateAt(i32, i32),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AiSubAction {
    Move(Cardinal),
    MoveTo(i32, i32),
    Actuate,
}
impl AiSubAction {
    /// Play this SubAction in the World, and return if it was successful
    /// Successful meaning that the state of the world changed.
    fn play(&self, ecs: &mut World, rsrc: &mut Resources) -> bool {
        let actions = match &self {
            AiSubAction::Move(cardinal) => try_move_player(*cardinal, ecs, rsrc),
            AiSubAction::MoveTo(x, y) => try_teleport_player(*x, *y, ecs, rsrc),
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
    start_time: time::Instant,
    searches: i32,
    seen: AiStatesCache,
    history: AiHistory,
    tested_action: AiAction,
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
            start_time: time::Instant::now(),
            searches: 0,
            seen: AiStatesCache::new(),
            history: AiHistory {
                possibilities: vec![],
            },
            tested_action: AiAction::ExitTo(0, 0),
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
        ctx.print(1, 28, format!("Cache Size: {}", self.seen.get_size()));
        ctx.print(
            1,
            29,
            format!(
                "SPS: {:.2}",
                self.searches as f32 / self.start_time.elapsed().as_secs_f32()
            ),
        );
        if self.finished {
            let txt = "Solved !";
            let start_x = (TERM_WIDTH - (txt.len() as i32 + 4)) / 2;
            let start_y = 6;
            ctx.draw_box(
                start_x,
                start_y,
                txt.len() + 4 - 1,
                2,
                RGB::named(YELLOW),
                RGB::named(BLACK),
            );
            ctx.print_color_centered(7, RGB::named(YELLOW), RGB::named(BLACK), txt);
        } else if self.paused {
            ctx.print_color_centered(7, RGB::named(YELLOW), RGB::named(BLACK), "Paused!");
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
                self.searches += 1;
                if self.seen.has_seen(ecs) {
                    self.sub_actions_success = false;
                    //println!("  DUP");
                    self.duplicates += 1;
                }
                if self.sub_actions_success {
                    let map = rsrc.get::<map::Map>().unwrap();
                    if map.is_impossible(&ecs) {
                        //self.paused = true;
                        self.dead_ends += 1;
                        self.sub_actions_success = false;
                        //println!("Dead end. Impossible map");
                    }
                }
                if self.sub_actions_success {
                    let mut possibilities = self.find_possible_actions(ecs, rsrc);
                    possibilities.shuffle(&mut thread_rng());
                    if possibilities.len() > 0 {
                        self.history.possibilities.push((cur_step, possibilities));
                    //println!("UPD {:?}", self.history.possibilities);
                    } else {
                        self.dead_ends += 1;
                        //println!("  DEAD-END");
                        self.sub_actions_success = false;
                    }
                }
                if !self.sub_actions_success {
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
                        //println!("    UNDO {}", undo_steps);
                    }
                }
                let (tested_action, sub_actions) = self
                    .history
                    .possibilities
                    .last_mut()
                    .unwrap()
                    .1
                    .pop()
                    .unwrap();
                //println!(
                //    "Depth {}. Doing {:?} : {:?}",
                //    self.history.possibilities.len(),
                //    tested_action,
                //    sub_actions
                //);
                //if self.history.possibilities.len() > 3 {
                //    self.paused = true;
                //}
                self.tested_action = tested_action;
                self.sub_actions = sub_actions;
            }
            if !self.sub_actions.is_empty() {
                let action = self.sub_actions.remove(0);
                self.sub_actions_success = true;
                action.play(ecs, rsrc);
                if !self.sub_actions_success {
                    //println!("  DEAD-END2 {:?}", action);
                    self.dead_ends += 1;
                }
                //println!("    DO {:?}", action)
            }
        }
        self.get_next_runstate(ctx)
    }
    fn get_next_runstate(&mut self, ctx: &mut BTerm) -> RunState {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => {
                    self.paused = !self.paused;
                    if !self.paused {
                        self.start_time = time::Instant::now();
                        self.searches = 0;
                    }
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
            RunState::GameAwaitingInput
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
                self.dead_ends += 1;
                self.sub_actions_success = false;
                //println!("  DEAD");
                RunState::GameAwaitingInput
            }
            TurnState::PlayerAtExit => {
                self.solutions += 1;
                self.sub_actions_success = false;
                //println!("  EXIT");
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
                    if map.is_blocked(movable_pos.x + dx, movable_pos.y + dy) {
                        // The direction is blocked, no point trying to move it.
                        continue;
                    }
                    let (invdx, invdy) = direction.inv().get_delta_xy();
                    let dest_x = movable_pos.x + invdx;
                    let dest_y = movable_pos.y + invdy;
                    if map.can_go_to((player_pos.x, player_pos.y), (dest_x, dest_y)) {
                        let mut sub_actions = vec![];
                        // Teleport
                        sub_actions.push(AiSubAction::MoveTo(dest_x, dest_y));
                        //Push action
                        sub_actions.push(AiSubAction::Move(*direction));
                        actions.push((
                            AiAction::PushAt(movable_pos.x, movable_pos.y, *direction),
                            sub_actions,
                        ));
                    }
                }
            }
            // Go to Activable and Activate
            let query2 = <(Read<Position>, Read<Actuator>)>::query();
            for (activable_pos, _actuator) in query2.iter(&ecs) {
                for direction in &[Cardinal::N, Cardinal::S, Cardinal::W, Cardinal::E] {
                    let (dx, dy) = direction.get_delta_xy();
                    let dest_x = activable_pos.x + dx;
                    let dest_y = activable_pos.y + dy;
                    if map.can_go_to((player_pos.x, player_pos.y), (dest_x, dest_y)) {
                        let mut sub_actions = vec![];
                        // Teleport
                        sub_actions.push(AiSubAction::MoveTo(dest_x, dest_y));
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
