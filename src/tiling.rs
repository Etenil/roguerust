use std::convert::From;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    StairsUp,
    StairsDown,
    Character(&'static str),
    Player,
}

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    visible: bool,
}

impl Tile {
    pub fn new(tile_type: TileType, visible: bool) -> Self {
        Tile { tile_type, visible }
    }

    pub fn get_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn visibility(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl From<TileType> for Tile {
    fn from(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            visible: false, // <--- TODO: this set the default beaviour
                            //            - true: all tiles of world and entities will be drawn
                            //            - false: only draw tiles visible for the player
        }
    }
}

pub struct TileGrid {
    grid: Vec<Vec<Tile>>,
    xsize: usize,
    ysize: usize,
}

impl TileGrid {
    pub fn new(xsize: usize, ysize: usize) -> TileGrid {
        let mut grid = TileGrid {
            grid: Vec::with_capacity(ysize),
            xsize,
            ysize,
        };

        for _ in 0..ysize {
            let mut subvec = Vec::with_capacity(xsize);
            for _ in 0..xsize {
                subvec.push(Tile::new(TileType::Empty, true));
            }
            grid.grid.push(subvec);
        }

        grid
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y][x] = tile;
    }

    /// Sets a tile if nothing lies underneath it.
    pub fn set_empty_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.set_tile(
            x,
            y,
            match self.grid[y][x].tile_type {
                TileType::Empty => tile,
                _ => self.grid[y][x].clone(),
            },
        )
    }

    pub fn raw_data(&self) -> &Vec<Vec<Tile>> {
        &self.grid
    }

    pub fn block_at(&self, x: usize, y: usize) -> &Tile {
        &self.grid[y + 1][x]
    }

    pub fn xsize(&self) -> usize {
        self.xsize
    }

    pub fn ysize(&self) -> usize {
        self.ysize
    }

    pub fn clear_fog_of_war(&mut self, center: &(usize, usize), radius: usize) {
        let startx: usize = 0.max(center.0 as isize - radius as isize) as usize;
        let starty: usize = 0.max(center.1 as isize - radius as isize) as usize;
        for x in startx..self.xsize.min(center.0 + radius) {
            for y in starty..self.ysize.min(center.1 + radius) {
                self.grid[y][x].visibility(true)
            }
        }
    }
}

pub fn tile_to_str(tile: &Tile) -> &str {
    if tile.is_visible() {
        match tile.tile_type {
            TileType::Floor => ".",
            TileType::Wall => "#",
            TileType::Empty => " ",
            TileType::StairsDown => ">",
            TileType::StairsUp => "<",
            TileType::Player => "@",
            TileType::Character(t) => t,
        }
    } else {
        " "
    }
}

pub trait Tileable {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String>;
}
