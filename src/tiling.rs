extern crate pancurses;

pub struct TileGrid {
    grid: Vec<Vec<TileType>>
}

impl TileGrid {
    pub fn new(xsize: usize, ysize: usize) -> TileGrid {
        let mut grid = TileGrid {
            grid: Vec::with_capacity(ysize)
        };

        for _ in 0..ysize {
            let mut subvec = Vec::with_capacity(xsize);
            for _ in 0..xsize {
                subvec.push(TileType::Empty);
            }
            grid.grid.push(subvec);
        }

        return grid;
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.grid[y][x] = tile;
    }

    /// Sets a tile if nothing lies underneath it.
    pub fn set_empty_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.set_tile(x, y, match self.grid[y][x] {
            TileType::Empty => tile,
            _ => self.grid[y][x].clone()
        })
    }

    pub fn raw_data(& self) -> & Vec<Vec<TileType>> {
        &self.grid
    }

    pub fn get_block_at(&self, x: usize, y: usize) -> &TileType {
        &self.grid[y + 1][x]
    }
}

pub fn tile_to_str(tile: &TileType) -> &str {
    match tile {
        TileType::Floor => ".",
        TileType::Wall => "#",
        TileType::Empty => " ",
        TileType::StairsDown => ">",
        TileType::StairsUp => "<",
        TileType::Player => "@",
        _ => "?"
    }
}

pub trait Tileable {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String>;
}

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    StairsUp,
    StairsDown,
    Character,
    Player
}
