use crate::components::{AtExit, Dead, Player, Position, Renderable};
use crate::map;
use bracket_lib::prelude::*;
use legion::prelude::*;

pub fn level_end_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("level_end_system")
        .read_resource::<map::Map>()
        .with_query(<(Read<Position>, Write<Renderable>)>::query().filter(tag::<Player>()))
        .build(|cmd, mut world, map, query| {
            for (entity, (position, mut renderable)) in query.iter_entities_mut(&mut world) {
                if map.is_lasered(position.x, position.y) {
                    cmd.add_tag(entity, Dead {});
                    renderable.fg = RGB::named(BROWN1);
                } else if map.is_exit(position.x, position.y) {
                    cmd.add_tag(entity, AtExit {});
                }
            }
        })
}
