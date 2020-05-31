use bracket_lib::prelude::*;
use legion::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cardinal {
    N,
    S,
    E,
    W,
    NE,
    NW,
}
impl Cardinal {
    pub fn get_delta_xy(&self) -> (i32, i32) {
        match self {
            Cardinal::N => (0, -1),
            Cardinal::S => (0, 1),
            Cardinal::E => (1, 0),
            Cardinal::W => (-1, 0),
            _ => panic!("Can only get_delta_xy for N,S,E,W"),
        }
    }
    pub fn inv(&self) -> Cardinal {
        match self {
            Cardinal::N => Cardinal::S,
            Cardinal::S => Cardinal::N,
            Cardinal::E => Cardinal::W,
            Cardinal::W => Cardinal::E,
            _ => panic!("Can only get_delta_xy for N,S,E,W"),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32, // 0 will be in front, masking 1 which will masked 2 etc...
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationKind {
    Laser,
    Weight,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Activable {
    pub active: bool,
    pub kind: ActivationKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Door {
    pub opened: bool,
    pub activations: Vec<Entity>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReflectsLaser {
    /// Direction the reflector is orientied: one of NEor NW
    pub orientation: Cardinal,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {}

#[derive(Clone, Debug, PartialEq)]
pub struct Laser {
    /// Direction the laser is firing at : one of N,S,E,W
    pub direction: Cardinal,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksTile {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksLaser {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movable {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actuator {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actuated {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UndoActuated {}
