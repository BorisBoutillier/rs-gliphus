use legion::prelude::*;
mod activable;
mod door;
mod map_indexing;

pub fn build_systems() -> Schedule {
    Schedule::builder()
        .add_system(map_indexing::map_indexing_system())
        .flush() // Following system need the map up to date
        .add_system(activable::activable_system())
        .flush() // Following system need active state up to date
        .add_system(door::door_system())
        .build()
}
