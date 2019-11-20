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
        Tile {
            tile_type,
            visible,
        }
    }

    pub fn get_type(&self) -> &TileType {
        &self.tile_type
    }
}

impl From<TileType> for Tile {
    fn from(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            visible: true,    // <--- TODO: this set the default beaviour
                              //            - true: all tiles of world and entities will be drawn
                              //            - false: only draw tiles visible for the player
        }
    }
}

pub struct TileGrid {
    grid: Vec<Vec<Tile>>,
}

impl TileGrid {
    pub fn new(xsize: usize, ysize: usize) -> TileGrid {
        let mut grid = TileGrid {
            grid: Vec::with_capacity(ysize),
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
}

pub fn tile_to_str(tile: &Tile) -> &str {
    if tile.visible {
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
