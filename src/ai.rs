use crate::{
    components::Cardinal,
    gui::{draw_ui, MainMenuSelection},
    map,
    player::{try_actuate, try_move_player},
    turn_history::{TurnState, TurnsHistory},
    RunState,
};
use bracket_lib::prelude::*;
use legion::prelude::*;

enum AiActions {
    MoveN,
    MoveS,
    MoveE,
    MoveW,
    Actuate,
}
impl AiActions {
    fn play(&self, ecs: &mut World, rsrc: &mut Resources) {
        let actions = match &self {
            AiActions::MoveN => try_move_player(Cardinal::N, ecs, rsrc),
            AiActions::MoveS => try_move_player(Cardinal::S, ecs, rsrc),
            AiActions::MoveE => try_move_player(Cardinal::E, ecs, rsrc),
            AiActions::MoveW => try_move_player(Cardinal::W, ecs, rsrc),
            AiActions::Actuate => try_actuate(ecs, rsrc),
        };
        if actions.len() > 0 {
            let mut turn_history = rsrc.get_mut::<TurnsHistory>().unwrap();
            turn_history.play_turn(ecs, actions);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AiState {
    Run,
    Pause,
}
pub struct AI {
    pub show: bool,
    pub deaths: u64,
    state: AiState,
}
impl AI {
    pub fn new() -> AI {
        AI {
            show: false,
            deaths: 0,
            state: AiState::Pause,
        }
    }
    pub fn draw_state(&self, rsrc: &Resources, ctx: &mut BTerm) {
        draw_ui(rsrc, ctx);
        ctx.print(20, 1, format!("Deaths : {}", self.deaths));
        if self.state == AiState::Pause {
            ctx.print_color_centered(6, RGB::named(RED1), RGB::named(LIGHT_GRAY), "Paused!");
        }
    }
    pub fn play_next_turn(
        &mut self,
        ecs: &mut World,
        rsrc: &mut Resources,
        ctx: &mut BTerm,
    ) -> RunState {
        if self.state == AiState::Run {
            let i;
            {
                let mut rng = rsrc.get_mut::<RandomNumberGenerator>().unwrap();
                i = rng.range(0, 5);
            }
            match i {
                0 => AiActions::MoveN.play(ecs, rsrc),
                1 => AiActions::MoveS.play(ecs, rsrc),
                2 => AiActions::MoveE.play(ecs, rsrc),
                3 => AiActions::MoveW.play(ecs, rsrc),
                _ => AiActions::Actuate.play(ecs, rsrc),
            }
        }
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => {
                    self.state = if self.state == AiState::Pause {
                        AiState::Run
                    } else {
                        AiState::Pause
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
        if self.state == AiState::Pause {
            RunState::GameDraw
        } else {
            RunState::GameTurn
        }
    }
    pub fn end_turn(&mut self, rsrc: &Resources) -> RunState {
        let curstate = rsrc.get::<TurnsHistory>().unwrap().state;
        match curstate {
            TurnState::PlayerDead => {
                let level = rsrc.get::<map::Map>().unwrap().level;
                self.deaths += 1;
                RunState::LoadLevel(level)
            }
            TurnState::PlayerAtExit => {
                let level = rsrc.get::<map::Map>().unwrap().level;
                RunState::LoadLevel(level + 1)
            }
            TurnState::Running => RunState::GameAwaitingInput,
        }
    }
}
