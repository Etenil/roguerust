use rand::Rng;

pub type Point = (usize, usize);

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    StairsUp,
    StairsDown
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
    start: Point,
    mid_point: Point,
    end: Point,
    corridor_dir: (i8, i8)
}

impl RoomEdge {
    pub fn new(start: Point, end: Point, corridor_dir: (i8, i8)) -> RoomEdge {
        RoomEdge {
            start,
            end,
            mid_point: (end.0 - start.0 / 2, end.1 - start.1 / 2),
            corridor_dir
        }
    }
}

struct Room {
    start: Point,
    center: Point,
    width: usize,
    height: usize,
    edges: [RoomEdge; 4]
}

impl Room {
    fn new(start: Point, width: usize, height: usize) -> Room {
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
    start: Point,
    length: usize,
    direction: CorridorType
}

impl Corridor {
    fn new(start: Point, length: usize, direction: CorridorType) -> Corridor {
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
            grid.set_empty_tile(x, y + 1, TileType::Wall);
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

pub struct Level {
    xsize: usize,
    ysize: usize,
    rooms: Vec<Room>,
    corridors: Vec<Corridor>
}

pub struct Dungeon {
    xsize: usize,
    ysize: usize,
    depth: usize,
    pub levels: Vec<Level>
}

pub trait Generable {
    fn generate(&mut self);
}

fn hor_dist(point1: Point, point2: Point) -> f32 {
    point2.0 as f32 - point1.0 as f32
}

fn ver_dist(point1: Point, point2: Point) -> f32 {
    point2.1 as f32 - point1.1 as f32
}

/// The distance between 2 points
fn distance(point1: Point, point2: Point) -> f32 {
    (
        hor_dist(point1, point2).powf(2.0)
        +
        ver_dist(point1, point2).powf(2.0)
    ).sqrt()
}

impl Dungeon {
    pub fn new(xsize: usize, ysize: usize, depth: usize) -> Dungeon {
        Dungeon {
            xsize,
            ysize,
            depth,
            levels: vec![]
        }
    }
}

impl Generable for Dungeon {
    fn generate(&mut self) {
        for _ in 0..self.depth {
            let mut level = Level::new(self.xsize, self.ysize, None);
            level.generate();
            self.levels.push(level);
        }
    }
}

impl Level {
    /// Creates a new level of horizontal size `xsize` and vertical size `ysize`.
    /// If start is Some<Point> then a room will be created at that point to link
    /// with an upper room.
    pub fn new(xsize: usize, ysize: usize, start: Option<Point>) -> Level {
        Level {
            xsize,
            ysize,
            rooms: Vec::new(),
            corridors: Vec::new()
        }
    }

    pub fn to_tilegrid(&self) -> Result<TileGrid, String> {
        let mut grid = TileGrid::new(self.xsize, self.ysize);

        for room in &self.rooms {
            room.tile(&mut grid)?;
        }

        for corridor in &self.corridors {
            corridor.tile(&mut grid)?;
        }

        Ok(grid)
    }

    pub fn get_start_point(&self) -> Point {
        if self.rooms.len() > 0 {
            return self.rooms[0].center;
        }
        return (0,0)
    }

    fn overlaps(&self, start: Point, width: usize, height: usize, padding: usize) -> bool {
        for room in &self.rooms {
            if room.start.0 < start.0 + width + padding &&
                room.start.0 + room.width + padding > start.0 &&
                room.start.1 < start.1 + height + padding &&
                room.start.1 + room.height + padding > start.1 {
                return true;
            }
        }

        return false;
    }

    fn room_distances(&self, point: Point) -> Vec<(usize, f32)> {
        let mut dists: Vec<(usize, f32)> = self.rooms
            .iter()
            .enumerate()
            .map(|(room_num, room): (usize, &Room)| -> (usize, f32) {
                (room_num, distance(point, room.center))
            })
            .collect();
        dists.sort_by(|(_, dista): &(usize, f32), (_, distb): &(usize, f32)| dista.partial_cmp(&distb).unwrap());
        dists
    }

    fn random_room(&self) -> Result<Room, String> {
        // TODO: Detect when not enough space is left to allocate a room.
        let mut rng = rand::thread_rng();
        let room_width = rng.gen_range(3, 12);
        let room_height = rng.gen_range(3, 12);

        // TODO: Find a way to write a lambda to generate the start point.
        let mut start: Point = (
            rng.gen_range(0, self.xsize - room_width),
            rng.gen_range(0, self.ysize - room_height)
        );

        while self.overlaps(start, room_width, room_height, 2) {
            start = (
                rng.gen_range(0, self.xsize - room_width),
                rng.gen_range(0, self.ysize - room_height)
            );
        }

        Ok(Room::new(start, room_width, room_height))
    }
}

impl Generable for Level {
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let room_number = rng.gen_range(3, 5);

        // Generate rooms
        for _ in 0..room_number {
            self.rooms.push(self.random_room().unwrap());
        }

        // Generate corridors
        for room in &self.rooms {
            // Find the nearest room.
            let distances = self.room_distances(room.center);
            let nearest_room = &self.rooms[distances[1].0];

            let mut xorigin = room.center;
            let xlength = hor_dist(room.center, nearest_room.center);
            if xlength < 0f32 {
                xorigin = nearest_room.center;
            }

            self.corridors.push(Corridor::new(
                xorigin,
                xlength.abs() as usize,
                CorridorType::Horizontal
            ));

            let angle_point = (xorigin.0 + xlength.abs() as usize, xorigin.1);
            let mut destination = nearest_room;
            if destination.center.1 == angle_point.1 {
                destination = room
            }
            let mut yorigin = angle_point;
            let ylength = ver_dist(yorigin, destination.center);
            if ylength < 0f32 {
                yorigin = destination.center;
            }

            self.corridors.push(Corridor::new(
                yorigin,
                ylength.abs() as usize,
                CorridorType::Vertical
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_world() {
        let mut level = Level::new(128, 128, None);
        level.generate();
    }
}
