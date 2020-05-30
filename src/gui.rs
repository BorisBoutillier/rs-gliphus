use crate::{map, turn_history::TurnsHistory, RunState, State, TERM_WIDTH};
use bracket_lib::prelude::*;

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

pub fn draw_ui(gs: &State, ctx: &mut BTerm) {
    let map = gs.rsrc.get::<map::Map>().unwrap();
    let turn_history = gs.rsrc.get::<TurnsHistory>().unwrap();
    ctx.print(1, 1, format!("Level No {}", map.level));
    ctx.print(1, 2, format!("Steps  : {}", turn_history.steps));
    ctx.print(1, 3, format!("Energy : {}", turn_history.energy_used));
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewPlayerGame,
    NewAiGame,
    Continue,
    Quit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}
pub fn main_menu(gs: &mut State, ctx: &mut BTerm) -> MainMenuResult {
    let runstate = gs.rsrc.get::<RunState>().unwrap();

    ctx.print_color_centered(11, RGB::named(YELLOW), RGB::named(BLACK), "Griphus");

    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        let color = if selection == MainMenuSelection::NewPlayerGame {
            RGB::named(MAGENTA)
        } else {
            RGB::named(WHITE)
        };
        ctx.print_color_centered(14, color, RGB::named(BLACK), "New Game");
        let color = if selection == MainMenuSelection::NewAiGame {
            RGB::named(MAGENTA)
        } else {
            RGB::named(WHITE)
        };
        ctx.print_color_centered(15, color, RGB::named(BLACK), "New AI Game");
        let color = if selection == MainMenuSelection::Continue {
            RGB::named(MAGENTA)
        } else {
            RGB::named(WHITE)
        };
        ctx.print_color_centered(16, color, RGB::named(BLACK), "Continue");
        let color = if selection == MainMenuSelection::Quit {
            RGB::named(MAGENTA)
        } else {
            RGB::named(WHITE)
        };
        ctx.print_color_centered(17, color, RGB::named(BLACK), "Quit");

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
                    let newselection;
                    match selection {
                        MainMenuSelection::NewPlayerGame => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::NewAiGame => {
                            newselection = MainMenuSelection::NewPlayerGame
                        }
                        MainMenuSelection::Continue => newselection = MainMenuSelection::NewAiGame,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::Continue,
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Down => {
                    let newselection;
                    match selection {
                        MainMenuSelection::NewPlayerGame => {
                            newselection = MainMenuSelection::NewAiGame
                        }
                        MainMenuSelection::NewAiGame => newselection = MainMenuSelection::Continue,
                        MainMenuSelection::Continue => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::NewPlayerGame,
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
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

    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewPlayerGame,
    }
}
