use crate::components::{BlocksTile, Door, Position};
use crate::map;
use legion::prelude::*;

pub fn map_indexing_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("map_indexing_system")
        .write_resource::<map::Map>()
        .with_query(<(Read<Position>,)>::query().filter(tag::<BlocksTile>()))
        .with_query(<(Read<Position>, Read<Door>)>::query())
        .build(|_, world, map, (query1, query2)| {
            map.reset_blocked();
            for (position,) in query1.iter(&world) {
                map.set_blocked(position.x, position.y);
            }
            for (position, door) in query2.iter(&world) {
                if !door.opened {
                    map.set_blocked(position.x, position.y);
                }
            }
        })
}
