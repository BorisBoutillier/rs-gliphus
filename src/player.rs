use super::State;
use crate::components::{Player, Position};
use crate::map;
use crate::RunState;
use bracket_lib::prelude::*;
use legion::prelude::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State) {
    let query = <(Read<Position>,)>::query().filter(tag::<Player>());
    let map = gs.rsrc.get::<map::Map>().unwrap();
    let mut to_move = vec![];
    for (player_entity, (pos,)) in query.iter_entities(&gs.ecs) {
        let dest_x = (pos.x + delta_x).max(0).min(79);
        let dest_y = (pos.y + delta_y).max(0).min(49);
        if !map.is_blocked(dest_x, dest_y) {
            to_move.push((player_entity, (dest_x, dest_y)));
        } else if let Some(movable_entity) = map.movable(dest_x, dest_y, &gs) {
            let moved_dest_x = dest_x + delta_x;
            let moved_dest_y = dest_y + delta_y;
            if !map.is_blocked(moved_dest_x, moved_dest_y) {
                to_move.push((player_entity, (dest_x, dest_y)));
                to_move.push((movable_entity, (moved_dest_x, moved_dest_y)));
            }
        }
    }
    for (entity, (x, y)) in to_move.into_iter() {
        let mut pos = gs.ecs.get_component_mut::<Position>(entity).unwrap();
        pos.x = x;
        pos.y = y;
    }
}

pub fn game_turn_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => {
            return RunState::GameAwaitingInput;
        }
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, gs),
            VirtualKeyCode::Right => try_move_player(1, 0, gs),
            VirtualKeyCode::Up => try_move_player(0, -1, gs),
            VirtualKeyCode::Down => try_move_player(0, 1, gs),
            VirtualKeyCode::Escape => return RunState::Menu,
            _ => {
                return RunState::GameAwaitingInput;
            }
        },
    }
    RunState::GameTurn
}
