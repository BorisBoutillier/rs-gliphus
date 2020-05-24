use legion::prelude::*;
use rltk;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cardinal {
    N,
    S,
    E,
    W,
}
// Components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
    pub render_order: i32, // 0 will be in front, masking 1 which will masked 2 etc...
}
#[derive(Clone, Debug, PartialEq)]
pub struct Laser {
    pub direction: Cardinal,
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

// Tags
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksTile {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksLaser {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movable {}
