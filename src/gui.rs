use crate::{map, turn_history::TurnsHistory, RunState, State, TERM_WIDTH};
use bracket_lib::prelude::*;
use legion::prelude::Resources;

pub fn draw_dead(_gs: &State, ctx: &mut BTerm) {
    let txt = "You died !";
    let start_x = (TERM_WIDTH - (txt.len() as i32 + 4)) / 2;
    let start_y = 5;
    ctx.draw_box(
        start_x,
        start_y,
        txt.len() + 4 - 1,
        2,
        RGB::named(RED1),
        RGB::named(BLACK),
    );
    ctx.print_color(
        (TERM_WIDTH - txt.len() as i32) / 2,
        start_y + 1,
        RGB::named(RED1),
        RGB::named(BLACK),
        txt,
    );
    let txt = "<ENTER to go to the menu>";
    ctx.print_color(
        (TERM_WIDTH - txt.len() as i32) / 2,
        start_y + 4,
        RGB::named(WHITE),
        RGB::named(BLACK),
        txt,
    );
}
pub fn draw_level_solved(_gs: &State, ctx: &mut BTerm) {
    let txt = "Level solved !";
    let start_x = (TERM_WIDTH - (txt.len() as i32 + 4)) / 2;
    let start_y = 5;
    ctx.draw_box(
        start_x,
        start_y,
        txt.len() + 4 - 1,
        2,
        RGB::named(PINK),
        RGB::named(BLACK),
    );
    ctx.print_color(
        (TERM_WIDTH - txt.len() as i32) / 2,
        start_y + 1,
        RGB::named(PINK),
        RGB::named(BLACK),
        txt,
    );
    let txt = "<ENTER to go to next level>";
    ctx.print_color(
        (TERM_WIDTH - txt.len() as i32) / 2,
        start_y + 4,
        RGB::named(WHITE),
        RGB::named(BLACK),
        txt,
    );
}

pub fn game_end_dead_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => RunState::GameDraw,
        Some(key) => match key {
            VirtualKeyCode::Return => RunState::MainMenu {
                menu_selection: MainMenuSelection::NewPlayerGame,
            },
            VirtualKeyCode::R => {
                let map = gs.rsrc.get::<map::Map>().unwrap();
                RunState::LoadLevel(map.level)
            }
            VirtualKeyCode::Back => {
                let mut turn_history = gs.rsrc.get_mut::<TurnsHistory>().unwrap();
                turn_history.undo_last_turn(&mut gs.ecs);
                RunState::GameDraw
            }
            _ => RunState::GameDraw,
        },
    }
}
pub fn game_level_end_input(gs: &State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => RunState::GameDraw,
        Some(key) => match key {
            VirtualKeyCode::Return => {
                let map = gs.rsrc.get::<map::Map>().unwrap();
                RunState::LoadLevel(map.level + 1)
            }
            _ => RunState::GameDraw,
        },
    }
}

pub fn draw_ui(rsrc: &Resources, ctx: &mut BTerm) {
    let map = rsrc.get::<map::Map>().unwrap();
    let turn_history = rsrc.get::<TurnsHistory>().unwrap();
    ctx.print(1, 1, format!("Level : {}", map.level));
    ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewPlayerGame = 0,
    NewAiGame = 1,
    Continue = 2,
    Quit = 3,
}
impl MainMenuSelection {
    fn get_name(&self) -> String {
        String::from(match self {
            MainMenuSelection::NewPlayerGame => "New Game",
            MainMenuSelection::NewAiGame => "New AI Game",
            MainMenuSelection::Continue => "Continue",
            MainMenuSelection::Quit => "Quit",
        })
    }
    fn print(&self, y: i32, selection: MainMenuSelection, ctx: &mut BTerm) {
        let fg = if &selection == self {
            RGB::named(MAGENTA)
        } else {
            RGB::named(WHITE)
        };
        ctx.print_color_centered(y, fg, RGB::named(BLACK), self.get_name());
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}
pub fn main_menu(
    ctx: &mut BTerm,
    selection: MainMenuSelection,
    can_continue: bool,
) -> MainMenuResult {
    ctx.print_color_centered(11, RGB::named(YELLOW), RGB::named(BLACK), "Griphus");

    let entries = if can_continue {
        vec![
            MainMenuSelection::NewPlayerGame,
            MainMenuSelection::NewAiGame,
            MainMenuSelection::Continue,
            MainMenuSelection::Quit,
        ]
    } else {
        vec![
            MainMenuSelection::NewPlayerGame,
            MainMenuSelection::NewAiGame,
            MainMenuSelection::Quit,
        ]
    };
    for (i, entry) in entries.iter().enumerate() {
        entry.print(14 + i as i32, selection, ctx);
    }
    match ctx.key {
        None => {
            return MainMenuResult::NoSelection {
                selected: selection,
            }
        }
        Some(key) => match key {
            VirtualKeyCode::Escape => {
                return MainMenuResult::NoSelection {
                    selected: MainMenuSelection::Quit,
                }
            }
            VirtualKeyCode::Up => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                return MainMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                };
            }
            VirtualKeyCode::Down => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                return MainMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                };
            }
            VirtualKeyCode::Return => {
                return MainMenuResult::Selected {
                    selected: selection,
                }
            }
            _ => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
        },
    }
}
