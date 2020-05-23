use super::State;
use crate::{
    components::{Activable, ActivationKind, BlocksTile, Door, Exit, Player, Position, Renderable},
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
                glyph: rltk::to_cp437('@'),
                fg: rltk::RGB::named(rltk::YELLOW),
                bg: rltk::RGB::named(rltk::BLACK),
                render_order: 0,
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
                glyph: rltk::to_cp437('x'),
                fg: rltk::RGB::named(rltk::RED),
                bg: rltk::RGB::named(rltk::BLACK),
                render_order: 2,
            },
        )],
    )[0]
    .clone();
    gs.ecs.insert(
        (Exit {},),
        vec![(
            Position { x: 5, y: 0 },
            Door {
                opened: false,
                activations: vec![weight_plate],
            },
            Renderable {
                glyph: rltk::to_cp437('D'),
                fg: rltk::RGB::named(rltk::RED),
                bg: rltk::RGB::named(rltk::BLACK),
                render_order: 1,
            },
        )],
    );
}
