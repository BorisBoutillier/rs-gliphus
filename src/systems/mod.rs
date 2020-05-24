use legion::prelude::*;
mod activable;
mod door;
mod laser;
mod level_end;
mod map_indexing;

pub fn build_systems() -> Schedule {
    Schedule::builder()
        .add_system(map_indexing::map_indexing_system())
        .add_system(laser::laser_system())
        .flush() // Following system need the map up to date
        .add_system(activable::activable_system())
        .flush() // Following system need active state up to date
        .add_system(door::door_system())
        .add_system(level_end::level_end_system())
        .build()
}
