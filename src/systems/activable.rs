use crate::components::{Activable, ActivationKind, Position, Renderable};
use crate::map;
use legion::prelude::*;

pub fn activable_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("activable_system")
        .read_resource::<map::Map>()
        .with_query(<(Write<Activable>, Write<Renderable>, Read<Position>)>::query())
        .build(|_, mut world, map, query| {
            for (mut activable, mut renderable, position) in query.iter_mut(&mut world) {
                match activable.kind {
                    ActivationKind::Weight => {
                        activable.active = map.is_blocked(position.x, position.y);
                    }
                    ActivationKind::Laser => {
                        activable.active = map.is_lasered(position.x, position.y);
                    }
                };
                renderable.fg = if activable.active {
                    rltk::RGB::named(rltk::GREEN)
                } else {
                    rltk::RGB::named(rltk::RED)
                };
            }
        })
}
