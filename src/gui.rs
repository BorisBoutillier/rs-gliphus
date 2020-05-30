use crate::{map, turn_history::TurnsHistory, RunState, State, TERM_HEIGHT, TERM_WIDTH};
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
pub fn draw_menu(ctx: &mut BTerm) {
    let txt = "PRESS ENTER TO PLAY";
    let start_x = (TERM_WIDTH - (txt.len() as i32 + 4)) / 2;
    let start_y = (TERM_HEIGHT - 3) / 2;
    ctx.draw_box(
        start_x,
        start_y,
        txt.len() + 4 - 1,
        2,
        RGB::named(YELLOW),
        RGB::named(BLACK),
    );
    ctx.print_color(
        (TERM_WIDTH - txt.len() as i32) / 2,
        start_y + 1,
        RGB::named(YELLOW),
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
        None => RunState::GameEndDead,
        Some(key) => match key {
            VirtualKeyCode::Return => RunState::Menu,
            VirtualKeyCode::Back => {
                let mut turn_history = gs.rsrc.get_mut::<TurnsHistory>().unwrap();
                turn_history.undo_last_turn(&mut gs.ecs);
                RunState::GameTurn
            }
            _ => RunState::GameEndDead,
        },
    }
}
pub fn game_level_end_input(gs: &State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => RunState::GameLevelEnd,
        Some(key) => match key {
            VirtualKeyCode::Return => {
                let map = gs.rsrc.get::<map::Map>().unwrap();
                RunState::LoadLevel(map.level + 1)
            }
            _ => RunState::GameLevelEnd,
        },
    }
}
pub fn menu_input(_gs: &State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => RunState::Menu,
        Some(key) => match key {
            VirtualKeyCode::Return => RunState::LoadLevel(1),
            _ => RunState::Menu,
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
