#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}
pub struct Map {
    tiles: Vec<TileType>,
    blocked_tiles: Vec<bool>,
    pub width: i32,
    pub height: i32,
}
impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; (width * height) as usize],
            blocked_tiles: vec![false; (width * height) as usize],
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
    pub fn set_tiletype(&mut self, x: i32, y: i32, tiletype: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tiletype;
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
    #[inline]
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
    #[inline]
    fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }
    pub fn draw(&self, ctx: &mut rltk::Rltk, start_x: i32, start_y: i32) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.idx_xy(idx);
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    ctx.set(
                        start_x + x,
                        start_y + y,
                        rltk::GRAY,
                        rltk::BLACK,
                        rltk::to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        start_x + x,
                        start_y + y,
                        rltk::BLUE_VIOLET,
                        rltk::BLACK,
                        rltk::to_cp437('#'),
                    );
                }
                TileType::Exit => {
                    ctx.set(
                        start_x + x,
                        start_y + y,
                        rltk::CYAN,
                        rltk::BLACK,
                        rltk::to_cp437('o'),
                    );
                }
            }
        }
    }
}
