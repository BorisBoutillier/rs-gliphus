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
    SE,
    SW,
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
