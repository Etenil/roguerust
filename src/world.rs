pub enum TileType {
    Empty,
    Wall,
    Floor,
}

enum CorridorType {
    Horizontal,
    Vertical
}

trait Tileable {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String>;
}

struct Room {
    start: (usize, usize),
    width: usize,
    height: usize
}

impl Room {
    fn new(start: (usize, usize), width: usize, height: usize) -> Room {
        Room {
            start,
            width,
            height
        }
    }
}

impl Tileable for Room {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String> {
        // TODO: Detect if the room would leave the grid.
        let endx = self.start.0 + self.width;
        let endy = self.start.1 + self.height;

        // Draw the walls
        for x in self.start.0..endx {
            grid.set_empty_tile(x, self.start.1, TileType::Wall);
            grid.set_empty_tile(x, endy, TileType::Wall);
        }

        for y in self.start.1..endy {
            grid.set_empty_tile(self.start.0, y, TileType::Wall);
            grid.set_empty_tile(endx, y, TileType::Wall);
        }

        // Fill the room
        for x in (self.start.0 + 1)..(endx) {
            for y in (self.start.1 + 1)..(endy) {
                grid.set_tile(x, y, TileType::Floor);
            }
        }

        Ok(())
    }
}

struct Corridor {
    start: (usize, usize),
    length: usize,
    direction: CorridorType
}

impl Corridor {
    fn new(start: (usize, usize), length: usize, direction: CorridorType) -> Corridor {
        Corridor {
            start,
            length,
            direction
        }
    }

    fn tile_vertical(&self, grid: &mut TileGrid) {
        let x = self.start.0;
        let endy = self.start.1 + self.length;
        for y in self.start.1..endy {
            grid.set_empty_tile(x - 1, y, TileType::Wall);
            grid.set_tile(x, y, TileType::Floor);
            grid.set_empty_tile(x + 1, y, TileType::Wall);
        }
    }

    fn tile_horizontal(&self, grid: &mut TileGrid) {
        let y = self.start.1;
        let endx = self.start.0 + self.length;
        for x in self.start.0..endx {
            grid.set_empty_tile(x, y - 1, TileType::Wall);
            grid.set_tile(x, y, TileType::Floor);
            grid.set_empty_tile(x, y - 1, TileType::Wall);
        }
    }
}

impl Tileable for Corridor {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String> {
        // TODO: ensure the corridor isn't leaving the grid.
        match self.direction {
            CorridorType::Horizontal => self.tile_horizontal(grid),
            CorridorType::Vertical => self.tile_vertical(grid)
        }
        Ok(())
    }
}

pub struct TileGrid {
    grid: Vec<Vec<TileType>>
}

impl TileGrid {
    pub fn new(size: usize) -> TileGrid {
        let mut grid = TileGrid {
            grid: Vec::with_capacity(size)
        };

        for _ in 0..size {
            let mut subvec = Vec::with_capacity(size);
            for _ in 0..size {
                subvec.push(TileType::Empty);
            }
            grid.grid.push(subvec);
        }

        return grid;
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.grid[y][x] = tile;
    }

    /// Sets a tile if nothing lies underneath it.
    fn set_empty_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.set_tile(x, y, match self.grid[y][x] {
            TileType::Empty => tile,
            _ => self.grid[y][x]
        })
    }
}

pub struct World {
    size: usize,
    world: Vec<Room>
}

pub trait GameWorld {
    fn new(size: usize) -> Self;

    fn generate(&mut self);

    fn to_tilegrid(&self) -> TileGrid;
}

impl World {

}

impl GameWorld for World {
    fn new(size: usize) -> World {
        World {
            size,
            world: Vec::new()
        }
    }
    
    fn generate(&mut self) {
        self.world.push(Room::new((3, 3), 5, 8));
    }

    fn to_tilegrid(&self) -> TileGrid {
        let mut grid = TileGrid::new(self.size);

        for room in self.world {
            room.tile(&mut grid);
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_world() {
        let mut world = World::new(128);
        world.generate();
    }
}
