use super::State;
use crate::glyphs::*;
use crate::{
    components::{
        Activable, ActivationKind, Actuator, BlocksLaser, BlocksTile, Cardinal, Door, Laser,
        Movable, Player, Position, ReflectsLaser, Renderable,
    },
    map,
};
use bracket_lib::prelude::*;

pub fn load_level(gs: &mut State, id: u64) {
    match id {
        1 => load_level_1(gs),
        _ => load_level_2(gs),
    }
}

pub fn load_level_1(gs: &mut State) {
    gs.ecs.delete_all();
    let mut map = map::Map::new(10, 10);
    map.set_tiletype(5, 0, map::TileType::Exit);
    gs.ecs.delete_all();
    gs.rsrc.insert(map);
    gs.ecs.insert(
        (Player {}, BlocksTile {}),
        vec![(
            Position { x: 5, y: 5 },
            Renderable {
                glyph: PLAYER,
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
                render_order: 0,
            },
        )],
    );
    gs.ecs.insert(
        (BlocksTile {}, Movable {}, BlocksLaser {}),
        vec![(
            Position { x: 6, y: 7 },
            Renderable {
                glyph: MOVABLE_BLOCK,
                fg: RGB::named(ORANGE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
    let weight_plate = gs.ecs.insert(
        (),
        vec![(
            Position { x: 2, y: 7 },
            Activable {
                active: false,
                kind: ActivationKind::Weight,
            },
            Renderable {
                glyph: WEIGHT_PLATE,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 2,
            },
        )],
    )[0]
    .clone();
    let laser_receptors = gs.ecs.insert(
        (BlocksTile {}, BlocksLaser {}),
        vec![
            (
                Position { x: 7, y: 2 },
                Activable {
                    active: false,
                    kind: ActivationKind::Laser,
                },
                Renderable {
                    glyph: LASER_RECEPTOR,
                    fg: RGB::named(ORANGE),
                    bg: RGB::named(BLACK),
                    render_order: 2,
                },
            ),
            (
                Position { x: 8, y: 5 },
                Activable {
                    active: false,
                    kind: ActivationKind::Laser,
                },
                Renderable {
                    glyph: LASER_RECEPTOR,
                    fg: RGB::named(ORANGE),
                    bg: RGB::named(BLACK),
                    render_order: 2,
                },
            ),
        ],
    );
    let laser_receptors = vec![laser_receptors[0].clone(), laser_receptors[1].clone()];
    gs.ecs.insert(
        (BlocksLaser {},),
        vec![(
            Position { x: 5, y: 0 },
            Door {
                opened: false,
                activations: vec![weight_plate, laser_receptors[0], laser_receptors[1]],
            },
            Renderable {
                glyph: DOOR_H_CLOSED,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
    gs.ecs.insert(
        (
            Laser {
                direction: Cardinal::N,
            },
            BlocksTile {},
            BlocksLaser {},
            Movable {},
        ),
        vec![(
            Position { x: 7, y: 8 },
            Renderable {
                glyph: LASER_N,
                fg: RGB::named(LIGHT_BLUE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
    gs.ecs.insert(
        (
            Laser {
                direction: Cardinal::E,
            },
            BlocksTile {},
            BlocksLaser {},
            Movable {},
        ),
        vec![(
            Position { x: 3, y: 4 },
            Renderable {
                glyph: LASER_E,
                fg: RGB::named(LIGHT_BLUE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
}

pub fn load_level_2(gs: &mut State) {
    gs.ecs.delete_all();
    let mut map = map::Map::new(15, 15);
    map.set_tiletype(5, 0, map::TileType::Exit);
    gs.rsrc.insert(map);
    gs.ecs.insert(
        (Player {}, BlocksTile {}),
        vec![(
            Position { x: 5, y: 5 },
            Renderable {
                glyph: PLAYER,
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
                render_order: 0,
            },
        )],
    );
    let laser_receptor = gs.ecs.insert(
        (BlocksLaser {},),
        vec![(
            Position { x: 7, y: 2 },
            Activable {
                active: false,
                kind: ActivationKind::Laser,
            },
            Renderable {
                glyph: LASER_RECEPTOR,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 2,
            },
        )],
    )[0]
    .clone();
    gs.ecs.insert(
        (BlocksLaser {},),
        vec![(
            Position { x: 5, y: 0 },
            Door {
                opened: false,
                activations: vec![laser_receptor],
            },
            Renderable {
                glyph: DOOR_H_CLOSED,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
    gs.ecs.insert(
        (BlocksTile {}, Movable {}, Actuator {}),
        vec![(
            Position { x: 10, y: 10 },
            Renderable {
                glyph: REFLECTOR_NE,
                fg: RGB::named(WHITE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
            ReflectsLaser {
                orientation: Cardinal::NE,
            },
        )],
    );
    gs.ecs.insert(
        (
            Laser {
                direction: Cardinal::N,
            },
            BlocksTile {},
            BlocksLaser {},
        ),
        vec![(
            Position { x: 2, y: 14 },
            Renderable {
                glyph: LASER_N,
                fg: RGB::named(LIGHT_BLUE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    );
}
