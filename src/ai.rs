use crate::{
    components::Cardinal,
    gui::MainMenuSelection,
    player::{try_actuate, try_move_player},
    turn_history::TurnsHistory,
    RunState,
};
use bracket_lib::prelude::*;
use legion::prelude::*;
use std::time;

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
    state: AiState,
    last_key_pressed: time::Instant,
}
impl AI {
    pub fn new() -> AI {
        AI {
            state: AiState::Run,
            last_key_pressed: time::Instant::now(),
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
        if self.last_key_pressed.elapsed() >= time::Duration::from_millis(100) {
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Space => {
                        self.state = if self.state == AiState::Pause {
                            AiState::Run
                        } else {
                            AiState::Pause
                        }
                    }
                    VirtualKeyCode::Escape => {
                        return RunState::MainMenu {
                            menu_selection: MainMenuSelection::Continue,
                        }
                    }
                    _ => {}
                }
                self.last_key_pressed = time::Instant::now();
            }
        }
        if self.state == AiState::Pause {
            RunState::GameAwaitingInput
        } else {
            RunState::GameTurn
        }
    }
}
