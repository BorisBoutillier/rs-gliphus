use crate::components::{BlocksTile, Door, Position};
use crate::map;
use legion::prelude::*;

pub fn map_indexing_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("map_indexing_system")
        .write_resource::<map::Map>()
        .with_query(<(Read<Position>,)>::query())
        .with_query(<(Read<Position>, Read<Door>)>::query())
        .build(|_, world, map, (query1, query2)| {
            map.reset_blocked();
            map.reset_content();
            for (entity, (position,)) in query1.iter_entities(&world) {
                map.add_content(position.x, position.y, entity);
                if world.get_tag::<BlocksTile>(entity).is_some() {
                    map.set_blocked(position.x, position.y);
                }
            }
            for (position, door) in query2.iter(&world) {
                if !door.opened {
                    map.set_blocked(position.x, position.y);
                }
            }
        })
}
