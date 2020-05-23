use super::State;
use crate::components::{Player, Position};
use crate::map;
use legion::prelude::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    let query = <(Write<Position>,)>::query().filter(tag::<Player>());
    let map = gs.rsrc.get::<map::Map>().unwrap();
    for (mut pos,) in query.iter_mut(&mut gs.ecs) {
        let dest_x = (pos.x + delta_x).max(0).min(79);
        let dest_y = (pos.y + delta_y).max(0).min(49);
        if !map.is_blocked(dest_x, dest_y) {
            pos.x = dest_x;
            pos.y = dest_y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut rltk::Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            rltk::VirtualKeyCode::Left => try_move_player(-1, 0, gs),
            rltk::VirtualKeyCode::Right => try_move_player(1, 0, gs),
            rltk::VirtualKeyCode::Up => try_move_player(0, -1, gs),
            rltk::VirtualKeyCode::Down => try_move_player(0, 1, gs),
            _ => {}
        },
    }
}
