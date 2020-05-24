use crate::components::{Actuated, Cardinal, Laser, Position, ReflectsLaser, Renderable};
use crate::{glyphs::*, map};
use legion::prelude::*;

pub fn reflector_actuation_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("reflector_acturation_system")
        .read_component::<ReflectsLaser>()
        .with_query(<(Write<ReflectsLaser>, Write<Renderable>)>::query().filter(tag::<Actuated>()))
        .build(|cmd, mut world, _, query| {
            for (entity, (mut reflector, mut renderable)) in query.iter_entities_mut(&mut world) {
                match reflector.orientation {
                    Cardinal::NE => {
                        reflector.orientation = Cardinal::NW;
                        renderable.glyph = REFLECTOR_NW;
                    }
                    _ => {
                        reflector.orientation = Cardinal::NE;
                        renderable.glyph = REFLECTOR_NE;
                    }
                };
                cmd.remove_tag::<Actuated>(entity);
            }
        })
}

pub fn laser_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("laser_system")
        .write_resource::<map::Map>()
        .read_component::<ReflectsLaser>()
        .with_query(<(Tagged<Laser>, Read<Position>)>::query())
        .build(|_, world, map, query| {
            map.reset_lasered();
            for (laser, pos) in query.iter(&world) {
                let mut direction = laser.direction;
                let mut cur_x = pos.x;
                let mut cur_y = pos.y;
                loop {
                    let (delta_x, delta_y) = delta_direction(direction);
                    cur_x += delta_x;
                    cur_y += delta_y;
                    map.set_lasered(cur_x, cur_y, direction);
                    if map.is_blocking_laser(cur_x, cur_y, world) {
                        break;
                    } else if let Some(orientation) = map.is_reflecting_laser(cur_x, cur_y, world) {
                        direction = match orientation {
                            Cardinal::NE => match direction {
                                Cardinal::N => Cardinal::E,
                                Cardinal::S => Cardinal::W,
                                Cardinal::E => Cardinal::N,
                                Cardinal::W => Cardinal::S,
                                _ => Cardinal::N,
                            },
                            Cardinal::NW => match direction {
                                Cardinal::N => Cardinal::W,
                                Cardinal::S => Cardinal::E,
                                Cardinal::W => Cardinal::N,
                                Cardinal::E => Cardinal::S,
                                _ => Cardinal::N,
                            },
                            _ => Cardinal::N,
                        }
                    }
                }
            }
        })
}

// Converts a laser direction to (delta_x,delta_y) to reach next tile
fn delta_direction(direction: Cardinal) -> (i32, i32) {
    match direction {
        Cardinal::N => (0, -1),
        Cardinal::S => (0, 1),
        Cardinal::E => (1, 0),
        Cardinal::W => (-1, 0),
        _ => panic!("Laser should only be N,S,E,W"),
    }
}
