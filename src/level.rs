use super::State;
use crate::glyphs::*;
use crate::{
    components::{
        Activable, ActivationKind, BlocksLaser, BlocksTile, Cardinal, Door, Laser, Movable, Player,
        Position, Renderable,
    },
    map,
};

pub fn load_level(gs: &mut State) {
    let mut map = map::Map::new(10, 10);
    map.set_tiletype(5, 0, map::TileType::Exit);
    gs.rsrc.insert(map);
    gs.ecs.insert(
        (Player {}, BlocksTile {}),
        vec![(
            Position { x: 5, y: 5 },
            Renderable {
                glyph: PLAYER,
                fg: rltk::RGB::named(rltk::YELLOW),
                bg: rltk::RGB::named(rltk::BLACK),
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
                fg: rltk::RGB::named(rltk::ORANGE),
                bg: rltk::RGB::named(rltk::BLACK),
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
                fg: rltk::RGB::named(rltk::RED),
                bg: rltk::RGB::named(rltk::BLACK),
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
                    fg: rltk::RGB::named(rltk::ORANGE),
                    bg: rltk::RGB::named(rltk::BLACK),
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
                    fg: rltk::RGB::named(rltk::ORANGE),
                    bg: rltk::RGB::named(rltk::BLACK),
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
                fg: rltk::RGB::named(rltk::RED),
                bg: rltk::RGB::named(rltk::BLACK),
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
                fg: rltk::RGB::named(rltk::LIGHT_BLUE),
                bg: rltk::RGB::named(rltk::BLACK),
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
                fg: rltk::RGB::named(rltk::LIGHT_BLUE),
                bg: rltk::RGB::named(rltk::BLACK),
                render_order: 1,
            },
        )],
    );
}
