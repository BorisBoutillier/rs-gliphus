use crate::components::{Player, Position, Renderable};
use crate::{
    map,
    turn_history::{TurnState, TurnsHistory},
};
use bracket_lib::prelude::*;
use legion::prelude::*;

pub fn level_end_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("level_end_system")
        .read_resource::<map::Map>()
        .write_resource::<TurnsHistory>()
        .with_query(<(Read<Position>, Write<Renderable>)>::query().filter(tag::<Player>()))
        .build(|_, mut world, (map, history), query| {
            for (position, mut renderable) in query.iter_mut(&mut world) {
                if map.is_lasered(position.x, position.y) {
                    history.state = TurnState::PlayerDead;
                    renderable.fg = RGB::named(BROWN1);
                } else if map.is_exit(position.x, position.y) {
                    history.state = TurnState::PlayerAtExit;
                }
            }
        })
}
