use super::State;
use crate::components::{Actuator, Cardinal, Player, Position};
use crate::map;
use crate::{
    gui::MainMenuSelection,
    turn_history::{Action, TurnsHistory},
    RunState,
};
use bracket_lib::prelude::*;
use legion::prelude::*;

pub fn try_move_player(direction: Cardinal, ecs: &mut World, rsrc: &mut Resources) -> Vec<Action> {
    let (delta_x, delta_y) = match direction {
        Cardinal::E => (1, 0),
        Cardinal::W => (-1, 0),
        Cardinal::N => (0, -1),
        Cardinal::S => (0, 1),
        _ => {
            panic!("Unexpected direction");
        }
    };
    let query = <(Read<Position>,)>::query().filter(tag::<Player>());
    let map = rsrc.get::<map::Map>().unwrap();
    let mut actions = vec![];
    for (player_entity, (pos,)) in query.iter_entities(&ecs) {
        let dest_x = (pos.x + delta_x).max(0).min(79);
        let dest_y = (pos.y + delta_y).max(0).min(49);
        if !map.is_blocked(dest_x, dest_y) {
            actions.push(Action::Moves(
                player_entity,
                (pos.x, pos.y),
                (dest_x, dest_y),
            ));
        } else if let Some(movable_entity) = map.movable(dest_x, dest_y, &ecs) {
            let moved_dest_x = dest_x + delta_x;
            let moved_dest_y = dest_y + delta_y;
            if !map.is_blocked(moved_dest_x, moved_dest_y) {
                actions.push(Action::Moves(
                    player_entity,
                    (pos.x, pos.y),
                    (dest_x, dest_y),
                ));
                actions.push(Action::Moves(
                    movable_entity,
                    (dest_x, dest_y),
                    (moved_dest_x, moved_dest_y),
                ));
                actions.push(Action::UseEnergy(1));
            }
        }
    }
    actions
}
pub fn try_actuate(ecs: &mut World, rsrc: &mut Resources) -> Vec<Action> {
    let query = <(Read<Position>,)>::query().filter(tag::<Player>());
    let map = rsrc.get::<map::Map>().unwrap();
    let mut actions = vec![];
    for (pos,) in query.iter(&ecs) {
        for (delta_x, delta_y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            for &entity in map.iter_content(pos.x + delta_x, pos.y + delta_y) {
                if ecs.get_component::<Actuator>(entity).is_some() {
                    actions.push(Action::Actuates(entity));
                    actions.push(Action::UseEnergy(1));
                }
            }
        }
    }
    actions
}

pub fn game_turn_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    let actions;
    match ctx.key {
        None => {
            return RunState::GameAwaitingInput;
        }
        Some(key) => match key {
            VirtualKeyCode::Left => {
                actions = try_move_player(Cardinal::W, &mut gs.ecs, &mut gs.rsrc)
            }
            VirtualKeyCode::Right => {
                actions = try_move_player(Cardinal::E, &mut gs.ecs, &mut gs.rsrc)
            }
            VirtualKeyCode::Up => actions = try_move_player(Cardinal::N, &mut gs.ecs, &mut gs.rsrc),
            VirtualKeyCode::Down => {
                actions = try_move_player(Cardinal::S, &mut gs.ecs, &mut gs.rsrc)
            }
            VirtualKeyCode::Space => actions = try_actuate(&mut gs.ecs, &mut gs.rsrc),
            VirtualKeyCode::Back => {
                let mut turn_history = gs.rsrc.get_mut::<TurnsHistory>().unwrap();
                turn_history.undo_last_turn(&mut gs.ecs);
                actions = vec![];
            }
            VirtualKeyCode::Escape => {
                return RunState::MainMenu {
                    menu_selection: MainMenuSelection::Continue,
                }
            }
            _ => {
                return RunState::GameAwaitingInput;
            }
        },
    }
    if actions.len() > 0 {
        let mut turn_history = gs.rsrc.get_mut::<TurnsHistory>().unwrap();
        turn_history.play_turn(&mut gs.ecs, actions);
    }
    RunState::GameTurn
}
