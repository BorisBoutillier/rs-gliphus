use crate::components::{Actuated, Player, Position, Renderable, UndoActuated};
use bracket_lib::prelude::*;
use legion::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Moves(Entity, (i32, i32), (i32, i32)), // Entity moved from x,y to x,y
    Actuates(Entity),                      // Entity has been actuated
    UseEnergy(i32),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    Running,
    PlayerDead,
    PlayerAtExit,
}

pub struct TurnsHistory {
    pub energy_used: i32,
    pub state: TurnState,
    pub history: Vec<Vec<Action>>,
}
impl TurnsHistory {
    pub fn new() -> TurnsHistory {
        TurnsHistory {
            energy_used: 0,
            state: TurnState::Running,
            history: vec![],
        }
    }
    pub fn play_turn(&mut self, ecs: &mut World, actions: Vec<Action>) {
        for &action in actions.iter() {
            match action {
                Action::Moves(entity, (_x1, _y1), (x2, y2)) => {
                    let mut pos = ecs.get_component_mut::<Position>(entity).unwrap();
                    pos.x = x2;
                    pos.y = y2;
                }
                Action::Actuates(entity) => {
                    ecs.add_tag(entity, Actuated {}).unwrap();
                }
                Action::UseEnergy(x) => {
                    self.energy_used += x;
                }
            }
        }
        self.history.push(actions);
    }
    pub fn undo_last_turn(&mut self, ecs: &mut World) {
        if let Some(actions) = self.history.pop() {
            for &action in actions.iter() {
                match action {
                    Action::Moves(entity, (x1, y1), (_x2, _y2)) => {
                        let mut pos = ecs.get_component_mut::<Position>(entity).unwrap();
                        pos.x = x1;
                        pos.y = y1;
                    }
                    Action::Actuates(entity) => {
                        ecs.add_tag(entity, UndoActuated {}).unwrap();
                    }
                    Action::UseEnergy(x) => {
                        self.energy_used -= x;
                    }
                }
            }
            if self.state == TurnState::PlayerDead {
                let query = <(Write<Renderable>,)>::query().filter(tag::<Player>());
                for (mut renderable,) in query.iter_mut(ecs) {
                    renderable.fg = RGB::named(YELLOW);
                }
            }
            self.state = TurnState::Running;
        }
    }
}
