use rand::Rng;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall,
    Corridor,
    Floor,
}

enum CorridorType {
    Horizontal,
    Vertical
}

trait Tileable {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String>;
}

const LEFT: (i8, i8) = (-1i8, 0);
const RIGHT: (i8, i8) = (1i8, 0);
const UP: (i8, i8) = (0, -1i8);
const DOWN: (i8, i8) = (0, 1i8);

struct RoomEdge {
    start: (usize, usize),
    mid_point: (usize, usize),
    end: (usize, usize),
    corridor_dir: (i8, i8)
}

impl RoomEdge {
    pub fn new(start: (usize, usize), end: (usize, usize), corridor_dir: (i8, i8)) -> RoomEdge {
        RoomEdge {
            start,
            end,
            mid_point: (end.0 - start.0 / 2, end.1 - start.1 / 2),
            corridor_dir
        }
    }
}

struct Room {
    start: (usize, usize),
    center: (usize, usize),
    width: usize,
    height: usize,
    edges: [RoomEdge; 4]
}

impl Room {
    fn new(start: (usize, usize), width: usize, height: usize) -> Room {
        Room {
            start,
            width,
            height,
            center: (start.0 + width / 2, start.1 + height / 2),
            edges: [
                RoomEdge::new(start, (start.0 + width, start.1), UP),
                RoomEdge::new(start, (start.0, start.1 + height), LEFT),
                RoomEdge::new((start.0, start.1 + height), (start.0 + width, start.1 + height), DOWN),
                RoomEdge::new((start.0 + width, start.1), (start.0 + width, start.1), RIGHT)
            ]
        }
    }
}

impl Tileable for Room {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String> {
        // TODO: Detect if the room would leave the grid.
        let endx = self.start.0 + self.width;
        let endy = self.start.1 + self.height;

        // Set the walls
        for x in self.start.0..(endx + 1) {
            grid.set_empty_tile(x, self.start.1, TileType::Wall);
            grid.set_empty_tile(x, endy, TileType::Wall);
        }

        for y in self.start.1..endy {
            grid.set_empty_tile(self.start.0, y, TileType::Wall);
            grid.set_empty_tile(endx, y, TileType::Wall);
        }

        // Fill the room
        for x in (self.start.0 + 1)..endx {
            for y in (self.start.1 + 1)..endy {
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

impl<'a> TileGrid {
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
            _ => self.grid[y][x].clone()
        })
    }

    pub fn raw_data(&'a self) -> &'a Vec<Vec<TileType>> {
        &self.grid
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
    fn overlaps(&self, start: (usize, usize), width: usize, height: usize, padding: usize) -> bool {
        for room in &self.world {
            if room.start.0 < start.0 + width + padding &&
                room.start.0 + room.width + padding > start.0 &&
                room.start.1 < start.1 + height + padding &&
                room.start.1 + room.height + padding > start.1 {
                return true;
            }
        }

        return false;
    }

    fn room_distances(&self, point: (usize, usize)) -> Vec<(usize, f32)> {
        let mut dists: Vec<(usize, f32)> = self.world
            .iter()
            .enumerate()
            .map(|(room_num, room): (usize, &Room)| -> (usize, f32) {
                (
                    room_num,
                    (
                        ((point.0 - room.center.0) as f32).powf(2.0)
                        +
                        ((point.1 - room.center.1) as f32).powf(2.0)
                    ).sqrt()
                )
            })
            .collect();
        dists.sort_by(|(_, dista): &(usize, f32), (_, distb): &(usize, f32)| dista.partial_cmp(&distb).unwrap());
        dists
    }

    fn random_room(&self) -> Result<Room, String> {
        // TODO: Detect when not enough space is left to allocate a room.
        let mut rng = rand::thread_rng();
        let room_width = rng.gen_range(3, 6);
        let room_height = rng.gen_range(3, 6);

        // TODO: Find a way to write a lambda to generate the start point.
        let mut start: (usize, usize) = (
            rng.gen_range(0, self.size - room_width),
            rng.gen_range(0, self.size - room_height)
        );

        while self.overlaps(start, room_width, room_height, 2) {
            start = (
                rng.gen_range(0, self.size - room_width),
                rng.gen_range(0, self.size - room_height)
            );
        }

        Ok(Room::new(start, room_width, room_height))
    }
}

impl GameWorld for World {
    fn new(size: usize) -> World {
        World {
            size,
            world: Vec::new()
        }
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let room_number = rng.gen_range(3, 5);

        for _ in 0..room_number {
            self.world.push(self.random_room().unwrap());
        }
    }

    fn to_tilegrid(&self) -> TileGrid {
        let mut grid = TileGrid::new(self.size);

        for room in &self.world {
            room.tile(&mut grid).unwrap();
        }

        for room in &self.world {
            // Find the nearest room.
            let distances = self.room_distances(room.center);
            let nearest_room = &self.world[distances[1].0];
            let mut corridor_start: (usize, usize);

            for edge in room.edges {

            }
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
