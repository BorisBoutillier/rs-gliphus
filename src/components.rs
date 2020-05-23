use legion::prelude::*;
use rltk;
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
pub struct Exit {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksTile {}
