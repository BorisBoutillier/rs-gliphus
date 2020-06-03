use crate::components::{Actuator, Movable, Player, Position};
use legion::prelude::*;
pub struct AiStatesCache {
    movables: Vec<Entity>,
    actuators: Vec<Entity>,
    seen: Vec<(Vec<Position>, Vec<(Position, Actuator)>)>,
}

impl AiStatesCache {
    pub fn new() -> AiStatesCache {
        AiStatesCache {
            movables: vec![],
            actuators: vec![],
            seen: vec![],
        }
    }
    pub fn init(&mut self, ecs: &World) {
        let query = <(Read<Position>,)>::query().filter(tag::<Movable>());
        for (entity, (_pos,)) in query.iter_entities(&ecs) {
            self.movables.push(entity);
        }
        // Also add the player, as same state but reached from different player position
        // is different
        let query = <(Read<Position>,)>::query().filter(tag::<Player>());
        for (entity, (_pos,)) in query.iter_entities(&ecs) {
            self.movables.push(entity);
        }
        let query = <(Read<Position>,)>::query().filter(component::<Actuator>());
        for (entity, (_pos,)) in query.iter_entities(&ecs) {
            self.actuators.push(entity);
        }
    }
    pub fn has_seen(&mut self, ecs: &World) -> bool {
        if self.seen.is_empty() {
            self.init(ecs);
        }
        let mut m = vec![];
        for &entity in self.movables.iter() {
            m.push(*(ecs.get_component::<Position>(entity).unwrap()));
        }
        let mut a = vec![];
        for &entity in self.actuators.iter() {
            let pos = ecs.get_component::<Position>(entity).unwrap();
            let act = ecs.get_component::<Actuator>(entity).unwrap();
            a.push((*pos, *act));
        }
        let entry = (m, a);
        let found = self.seen.contains(&entry);
        if found {
            true
        } else {
            self.seen.push(entry);
            false
        }
    }
    pub fn get_size(&self) -> usize {
        self.seen.len()
    }
}
