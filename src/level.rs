use super::State;
use crate::glyphs::*;
use crate::{
    components::{
        Activable, ActivationKind, Actuator, Block, BlocksLaser, BlocksTile, Cardinal, Door, Laser,
        Movable, Player, Position, ReflectsLaser, Renderable,
    },
    map,
    turn_history::TurnsHistory,
};
use bracket_lib::prelude::*;
use legion::prelude::*;
use map::TileType;
use std::fs;

pub fn load_level(gs: &mut State, level: u64) {
    match level {
        x if x <= 4 => load_level_from_file(level, &format!("resources/level_00{}.txt", level), gs),
        x if x >= 10001 && x <= 10006 => load_level_from_file(
            level,
            &format!("resources/ai_test_{}.txt", level - 10000),
            gs,
        ),
        _ => load_level_from_file(level, "resources/level_end.txt", gs),
    }
}

fn load_level_from_file(level: u64, file: &str, gs: &mut State) {
    let content = fs::read_to_string(file).unwrap();
    let lines = content.trim().split("\n").collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();
    gs.ecs.delete_all();
    let mut map = map::Map::new(level, width as i32, height as i32);
    let mut activations = vec![];
    let mut exit = (0, 0);
    for y in 0..height {
        for (x, c) in lines[y].chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match c {
                '#' => map.set_tiletype(x, y, TileType::Wall),
                '.' => map.set_tiletype(x, y, TileType::Floor),
                ' ' => map.set_tiletype(x, y, TileType::Floor),
                '*' => {
                    map.set_tiletype(x, y, TileType::Floor);
                    activations.push(spawn_weight_plate(gs, x, y));
                    spawn_block(gs, x, y);
                }
                'E' => {
                    map.set_tiletype(x, y, TileType::Exit);
                    exit = (x, y);
                }
                'e' => {
                    spawn_laser(gs, x, y, Cardinal::E);
                }
                'n' => {
                    spawn_laser(gs, x, y, Cardinal::N);
                }
                's' => {
                    spawn_laser(gs, x, y, Cardinal::S);
                }
                'w' => {
                    spawn_laser(gs, x, y, Cardinal::W);
                }
                '/' => {
                    spawn_laser_reflector(gs, x, y, Cardinal::NE);
                }
                '\\' => {
                    spawn_laser_reflector(gs, x, y, Cardinal::NW);
                }
                'x' => {
                    activations.push(spawn_weight_plate(gs, x, y));
                }
                'o' => {
                    activations.push(spawn_laser_receptor(gs, x, y));
                }
                'b' | '$' => {
                    spawn_block(gs, x, y);
                }
                '@' => {
                    spawn_player(gs, x, y);
                }
                x => println!("Unused {}", x),
            }
        }
    }
    spawn_door(gs, exit.0, exit.1, activations);
    gs.rsrc.insert(map);
    gs.rsrc.insert(TurnsHistory::new());
}

fn spawn_player(gs: &mut State, x: i32, y: i32) -> Entity {
    gs.ecs.insert(
        (Player {},), // BlocksTile {}),
        vec![(
            Position { x, y },
            Renderable {
                glyph: PLAYER,
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
                render_order: 0,
            },
        )],
    )[0]
}

fn spawn_laser(gs: &mut State, x: i32, y: i32, direction: Cardinal) -> Entity {
    gs.ecs.insert(
        (
            Laser { direction },
            BlocksTile {},
            BlocksLaser {},
            Movable {},
        ),
        vec![(
            Position { x, y },
            Renderable {
                glyph: match direction {
                    Cardinal::N => LASER_N,
                    Cardinal::E => LASER_E,
                    Cardinal::W => LASER_W,
                    _ => LASER_S,
                },
                fg: RGB::named(LIGHT_BLUE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    )[0]
}

fn spawn_block(gs: &mut State, x: i32, y: i32) -> Entity {
    gs.ecs.insert(
        (Block {}, BlocksTile {}, Movable {}, BlocksLaser {}),
        vec![(
            Position { x, y },
            Renderable {
                glyph: MOVABLE_BLOCK,
                fg: RGB::named(ORANGE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    )[0]
}

fn spawn_weight_plate(gs: &mut State, x: i32, y: i32) -> Entity {
    gs.ecs.insert(
        (),
        vec![(
            Position { x, y },
            Activable {
                active: false,
                kind: ActivationKind::Weight,
            },
            Renderable {
                glyph: WEIGHT_PLATE,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 2,
            },
        )],
    )[0]
}
fn spawn_laser_receptor(gs: &mut State, x: i32, y: i32) -> Entity {
    gs.ecs.insert(
        (BlocksTile {}, BlocksLaser {}),
        vec![(
            Position { x, y },
            Activable {
                active: false,
                kind: ActivationKind::Laser,
            },
            Renderable {
                glyph: LASER_RECEPTOR,
                fg: RGB::named(ORANGE),
                bg: RGB::named(BLACK),
                render_order: 2,
            },
        )],
    )[0]
}

fn spawn_door(gs: &mut State, x: i32, y: i32, activations: Vec<Entity>) -> Entity {
    gs.ecs.insert(
        (BlocksLaser {},),
        vec![(
            Position { x, y },
            Door {
                opened: false,
                activations: activations,
            },
            Renderable {
                glyph: DOOR_H_CLOSED,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
        )],
    )[0]
}

fn spawn_laser_reflector(gs: &mut State, x: i32, y: i32, orientation: Cardinal) -> Entity {
    gs.ecs.insert(
        (BlocksTile {}, Movable {}),
        vec![(
            Position { x, y },
            Renderable {
                glyph: if orientation == Cardinal::NE {
                    REFLECTOR_NE
                } else {
                    REFLECTOR_NW
                },
                fg: RGB::named(WHITE),
                bg: RGB::named(BLACK),
                render_order: 1,
            },
            ReflectsLaser {
                orientation: orientation,
            },
            Actuator {
                state: if orientation == Cardinal::NW { 0 } else { 1 },
            },
        )],
    )[0]
}
