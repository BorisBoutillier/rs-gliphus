use crate::components::{Cardinal, Laser, Position};
use crate::map;
use legion::prelude::*;

pub fn laser_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("laser_system")
        .write_resource::<map::Map>()
        .with_query(<(Tagged<Laser>, Read<Position>)>::query())
        .build(|_, world, map, query| {
            map.reset_lasered();
            for (laser, pos) in query.iter(&world) {
                let delta_x: i32;
                let delta_y: i32;
                match laser.direction {
                    Cardinal::N => {
                        delta_x = 0;
                        delta_y = -1;
                    }
                    Cardinal::S => {
                        delta_x = 0;
                        delta_y = 1;
                    }
                    Cardinal::E => {
                        delta_x = 1;
                        delta_y = 0;
                    }
                    Cardinal::W => {
                        delta_x = -1;
                        delta_y = 0;
                    }
                };
                let mut cur_x = pos.x;
                let mut cur_y = pos.y;
                loop {
                    cur_x += delta_x;
                    cur_y += delta_y;
                    map.set_lasered(cur_x, cur_y, laser.direction);
                    if map.is_blocking_laser(cur_x, cur_y, world) {
                        break;
                    }
                }
            }
        })
}
