use crate::{
    components::{BlocksLaser, BlocksTile, Cardinal, Movable, ReflectsLaser},
    glyphs::*,
};
use bracket_lib::prelude::*;
use legion::prelude::*;
use legion::systems::SubWorld;
use std::slice::Iter;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}
pub struct Map {
    pub level: u64,
    tiles: Vec<TileType>,
    blocked_tiles: Vec<bool>,
    content_tiles: Vec<Vec<Entity>>,
    lasered_tiles: Vec<Vec<Cardinal>>,
    pub width: i32,
    pub height: i32,
}
impl Map {
    pub fn empty() -> Map {
        Map {
            level: 0,
            tiles: vec![],
            blocked_tiles: vec![],
            content_tiles: vec![],
            lasered_tiles: vec![],
            width: 0,
            height: 0,
        }
    }
    pub fn new(level: u64, width: i32, height: i32) -> Map {
        let mut map = Map {
            level,
            tiles: vec![TileType::Floor; (width * height) as usize],
            blocked_tiles: vec![false; (width * height) as usize],
            content_tiles: vec![vec![]; (width * height) as usize],
            lasered_tiles: vec![vec![]; (width * height) as usize],
            width,
            height,
        };

        for x in 0..width {
            map.set_tiletype(x, 0, TileType::Wall);
            map.set_tiletype(x, height - 1, TileType::Wall);
        }
        for y in 0..height {
            map.set_tiletype(0, y, TileType::Wall);
            map.set_tiletype(width - 1, y, TileType::Wall);
        }
        map
    }
    pub fn is_exit(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] == TileType::Exit
    }
    pub fn set_tiletype(&mut self, x: i32, y: i32, tiletype: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tiletype;
    }
    pub fn reset_lasered(&mut self) {
        for lasered_tile in self.lasered_tiles.iter_mut() {
            lasered_tile.clear();
        }
    }
    pub fn set_lasered(&mut self, x: i32, y: i32, direction: Cardinal) {
        let direction = match direction {
            Cardinal::N | Cardinal::S => Cardinal::N,
            Cardinal::E | Cardinal::W => Cardinal::E,
            _ => panic!("Lasert should be only N,S,E,W"),
        };
        let idx = self.xy_idx(x, y);
        if !self.lasered_tiles[idx].contains(&direction) {
            self.lasered_tiles[idx].push(direction);
        }
    }
    pub fn is_lasered(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        self.lasered_tiles[idx].len() != 0
    }
    pub fn reset_blocked(&mut self) {
        for (idx, blocked_tile) in self.blocked_tiles.iter_mut().enumerate() {
            *blocked_tile = self.tiles[idx] == TileType::Wall;
        }
    }
    pub fn set_blocked(&mut self, x: i32, y: i32) {
        let idx = self.xy_idx(x, y);
        self.blocked_tiles[idx] = true;
    }
    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        self.blocked_tiles[idx]
    }
    pub fn is_blocking_laser(&self, x: i32, y: i32, ecs: &SubWorld) -> bool {
        let idx = self.xy_idx(x, y);
        if self.tiles[idx] == TileType::Wall {
            return true;
        }
        for &entity in self.content_tiles[idx].iter() {
            if ecs.get_tag::<BlocksLaser>(entity).is_some() {
                return true;
            }
        }
        false
    }
    pub fn is_reflecting_laser(&self, x: i32, y: i32, ecs: &SubWorld) -> Option<Cardinal> {
        let idx = self.xy_idx(x, y);
        for &entity in self.content_tiles[idx].iter() {
            if let Some(reflector) = ecs.get_component::<ReflectsLaser>(entity) {
                return Some(reflector.orientation);
            }
        }
        None
    }
    pub fn reset_content(&mut self) {
        for content in self.content_tiles.iter_mut() {
            content.clear();
        }
    }
    pub fn add_content(&mut self, x: i32, y: i32, entity: Entity) {
        let idx = self.xy_idx(x, y);
        self.content_tiles[idx].push(entity);
    }
    pub fn iter_content(&self, x: i32, y: i32) -> Iter<Entity> {
        let idx = self.xy_idx(x, y);
        self.content_tiles[idx].iter()
    }
    /// Among the entities on the tile, the return the one that is BlocksTile and Movable if it exists.
    pub fn movable(&self, x: i32, y: i32, ecs: &World) -> Option<Entity> {
        let idx = self.xy_idx(x, y);
        for &entity in self.content_tiles[idx].iter() {
            if ecs.get_tag::<BlocksTile>(entity).is_some()
                && ecs.get_tag::<Movable>(entity).is_some()
            {
                return Some(entity);
            }
        }
        None
    }
    #[inline]
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
    #[inline]
    fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }
    pub fn draw(&self, ctx: &mut BTerm, start_x: i32, start_y: i32) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.idx_xy(idx);
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    let glyph = if self.lasered_tiles[idx].contains(&Cardinal::N) {
                        if self.lasered_tiles[idx].contains(&Cardinal::E) {
                            LASERED_NS_EW
                        } else {
                            LASERED_NS
                        }
                    } else if self.lasered_tiles[idx].contains(&Cardinal::E) {
                        LASERED_EW
                    } else {
                        FLOOR
                    };
                    ctx.set(start_x + x, start_y + y, GRAY, BLACK, glyph);
                }
                TileType::Wall => {
                    ctx.set(start_x + x, start_y + y, BLUE_VIOLET, BLACK, WALL);
                }
                TileType::Exit => {
                    ctx.set(start_x + x, start_y + y, CYAN, BLACK, EXIT);
                }
            }
        }
    }
}
