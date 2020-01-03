use crate::entities::{Character, Enemy, Entity};
use crate::tiling::{Tile, TileGrid, TileType, Tileable};
use rand::Rng;
use std::cmp::{min, PartialEq};
use std::fmt;

pub type Point = (usize, usize);
pub type Movement = (i8, i8);

#[derive(PartialEq)]
enum CorridorType {
    Horizontal,
    Vertical,
}

pub const LEFT: Movement = (-1, 0);
pub const RIGHT: Movement = (1, 0);
pub const UP: Movement = (0, -1);
pub const DOWN: Movement = (0, 1);

pub fn apply_movement(point: Point, movement: Movement) -> Result<Point, String> {
    let x = point.0 as i32 + movement.0 as i32;
    let y = point.1 as i32 + movement.1 as i32;
    if x < 0 || y < 0 {
        return Err(String::from("Can't move point off screen"));
    }
    Ok((x as usize, y as usize))
}

struct Room {
    start: Point,
    center: Point,
    width: usize,
    height: usize,
}

impl Room {
    fn new(start: Point, width: usize, height: usize) -> Room {
        Room {
            start,
            width,
            height,
            center: (
                start.0 + (width as f32 / 2.0).floor() as usize,
                start.1 + (height as f32 / 2.0).floor() as usize,
            ),
        }
    }
}

impl Tileable for Room {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String> {
        let endx = self.start.0 + self.width;
        let endy = self.start.1 + self.height;

        if endx >= grid.xsize() || endy > grid.ysize() {
            return Err(String::from("Room outside of grid bounds"));
        }

        // Set the walls
        for x in self.start.0..=endx {
            grid.set_empty_tile(x, self.start.1, Tile::from(TileType::Wall));
            grid.set_empty_tile(x, endy, Tile::from(TileType::Wall));
        }

        for y in self.start.1..endy {
            grid.set_empty_tile(self.start.0, y, Tile::from(TileType::Wall));
            grid.set_empty_tile(endx, y, Tile::from(TileType::Wall));
        }

        // Fill the room
        for x in (self.start.0 + 1)..endx {
            for y in (self.start.1 + 1)..endy {
                grid.set_tile(x, y, Tile::from(TileType::Floor));
            }
        }

        Ok(())
    }
}

#[derive(PartialEq)]
struct Corridor {
    start: Point,
    length: usize,
    direction: CorridorType,
}

impl fmt::Debug for Corridor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} corridor from ({},{}) of length {}",
            match self.direction {
                CorridorType::Horizontal => "Horizontal",
                CorridorType::Vertical => "Vertical",
            },
            self.start.0,
            self.start.1,
            self.length
        )
    }
}

impl Corridor {
    fn new(start: Point, length: usize, direction: CorridorType) -> Corridor {
        Corridor {
            start,
            length,
            direction,
        }
    }

    pub fn make(start: Point, end: Point) -> Result<Corridor, String> {
        if start.0 != end.0 && start.1 != end.1 {
            return Err(String::from(
                "Start and end points must be aligned to make a corridor",
            ));
        }

        let (dir, length) = if start.0 == end.0 {
            (
                CorridorType::Vertical,
                start.1.max(end.1) - start.1.min(end.1),
            )
        } else {
            (
                CorridorType::Horizontal,
                start.0.max(end.0) - start.0.min(end.0),
            )
        };

        if length == 0 {
            return Err(String::from("Can't create 0-length corridor"));
        }

        let origin = match dir {
            CorridorType::Horizontal => {
                if start.0 < end.0 {
                    start
                } else {
                    end
                }
            }
            CorridorType::Vertical => {
                if start.1 < end.1 {
                    start
                } else {
                    end
                }
            }
        };

        Ok(Corridor::new(origin, length, dir))
    }

    pub fn link(start: Point, end: Point) -> Result<Vec<Corridor>, String> {
        if start.0 == end.0 || start.1 == end.1 {
            return Ok(vec![Corridor::make(start, end)?]);
        }
        let mut rng = rand::thread_rng();
        let start_hor = rng.gen_bool(0.5);
        let angle_point = if start_hor {
            (end.0, start.1)
        } else {
            (start.0, end.1)
        };

        Ok(vec![
            Corridor::make(start, angle_point)?,
            Corridor::make(angle_point, end)?,
        ])
    }

    fn tile_vertical(&self, grid: &mut TileGrid) {
        let x = self.start.0;
        let endy = self.start.1 + self.length;
        for y in self.start.1..endy {
            grid.set_empty_tile(x - 1, y, Tile::from(TileType::Wall));
            grid.set_tile(x, y, Tile::from(TileType::Floor));
            grid.set_empty_tile(x + 1, y, Tile::from(TileType::Wall));
        }
        // Wall ends
        grid.set_empty_tile(x - 1, self.start.1, Tile::from(TileType::Wall));
        grid.set_empty_tile(x, self.start.1, Tile::from(TileType::Wall));
        grid.set_empty_tile(x + 1, self.start.1, Tile::from(TileType::Wall));
        grid.set_empty_tile(x - 1, endy, Tile::from(TileType::Wall));
        grid.set_empty_tile(x, endy, Tile::from(TileType::Wall));
        grid.set_empty_tile(x + 1, endy, Tile::from(TileType::Wall));
    }

    fn tile_horizontal(&self, grid: &mut TileGrid) {
        let y = self.start.1;
        let endx = self.start.0 + self.length;
        for x in self.start.0..endx {
            grid.set_empty_tile(x, y - 1, Tile::from(TileType::Wall));
            grid.set_tile(x, y, Tile::from(TileType::Floor));
            grid.set_empty_tile(x, y + 1, Tile::from(TileType::Wall));
        }
        // Wall ends
        grid.set_empty_tile(self.start.0, y - 1, Tile::from(TileType::Wall));
        grid.set_empty_tile(self.start.0, y, Tile::from(TileType::Wall));
        grid.set_empty_tile(self.start.0, y + 1, Tile::from(TileType::Wall));
        grid.set_empty_tile(endx, y - 1, Tile::from(TileType::Wall));
        grid.set_empty_tile(endx, y, Tile::from(TileType::Wall));
        grid.set_empty_tile(endx, y + 1, Tile::from(TileType::Wall));
    }
}

impl Tileable for Corridor {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String> {
        // TODO: ensure the corridor isn't leaving the grid.
        match self.direction {
            CorridorType::Horizontal => self.tile_horizontal(grid),
            CorridorType::Vertical => self.tile_vertical(grid),
        }
        Ok(())
    }
}

pub struct Level {
    xsize: usize,
    ysize: usize,
    depth: usize,
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
    pub entities: Vec<Box<dyn Entity>>,
    entrance: Point,
    exit: Point,
}

pub struct Dungeon {
    xsize: usize,
    ysize: usize,
    depth: usize,
    pub levels: Vec<Level>,
}

pub trait Generatable {
    fn generate(&mut self);
}

impl Dungeon {
    pub fn new(xsize: usize, ysize: usize, depth: usize) -> Dungeon {
        Dungeon {
            xsize,
            ysize,
            depth,
            levels: vec![],
        }
    }

    pub fn xsize(&self) -> usize {
        self.xsize
    }

    pub fn ysize(&self) -> usize {
        self.ysize
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl Generatable for Dungeon {
    fn generate(&mut self) {
        let mut level = Level::new(self.xsize, self.ysize, 1, None);
        level.generate();
        let mut next_entrance = level.exit();
        self.levels.push(level);

        for d in 1..self.depth {
            level = Level::new(self.xsize, self.ysize, d + 1, Some(next_entrance));
            level.generate();
            next_entrance = level.exit();
            self.levels.push(level);
        }
    }
}

impl Level {
    /// Creates a new level of horizontal size `xsize` and vertical size `ysize`.
    /// If start is Some<Point> then a room will be created at that point to link
    /// with an upper room.
    pub fn new(xsize: usize, ysize: usize, depth: usize, start: Option<Point>) -> Level {
        Level {
            xsize,
            ysize,
            rooms: vec![],
            corridors: vec![],
            entities: vec![],
            entrance: match start {
                Some(st) => st,
                None => (0, 0),
            },
            exit: (0, 0),
            depth,
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

        grid.set_tile(
            self.entrance.0,
            self.entrance.1,
            Tile::from(TileType::StairsUp),
        );
        grid.set_tile(self.exit.0, self.exit.1, Tile::from(TileType::StairsDown));

        Ok(grid)
    }

    pub fn start_point(&self) -> Point {
        if !self.rooms.is_empty() {
            return self.rooms[0].center;
        }
        (0, 0)
    }

    // pub fn entrance(&self) -> Point {
    //     self.entrance
    // }

    pub fn exit(&self) -> Point {
        self.exit
    }

    fn overlaps(&self, start: Point, width: usize, height: usize, padding: usize) -> bool {
        for room in &self.rooms {
            if room.start.0 < start.0 + width + padding
                && room.start.0 + room.width + padding > start.0
                && room.start.1 < start.1 + height + padding
                && room.start.1 + room.height + padding > start.1
            {
                return true;
            }
        }

        false
    }

    fn random_room(&self) -> Result<Room, String> {
        // TODO: Detect when not enough space is left to allocate a room.
        let mut rng = rand::thread_rng();
        let room_width = rng.gen_range(4, 12);
        let room_height = rng.gen_range(4, 12);

        // TODO: Find a way to write a lambda to generate the start point.
        let mut start: Point = (
            rng.gen_range(0, self.xsize - room_width),
            rng.gen_range(0, self.ysize - room_height),
        );

        while self.overlaps(start, room_width, room_height, 2) {
            start = (
                rng.gen_range(0, self.xsize - room_width),
                rng.gen_range(0, self.ysize - room_height),
            );
        }

        Ok(Room::new(start, room_width, room_height))
    }

    fn centered_room(&self, center: Point) -> Room {
        let mut rng = rand::thread_rng();
        let room_width: usize =
            rng.gen_range(3, min(min(12, (self.xsize - center.0) * 2), center.0 * 2));
        let room_height: usize =
            rng.gen_range(3, min(min(12, (self.ysize - center.1) * 2), center.1 * 2));

        let start = (
            (center.0 as f32 - (room_width as f32 / 2f32)).floor() as usize,
            (center.1 as f32 - (room_height as f32 / 2f32)).floor() as usize,
        );

        Room::new(start, room_width, room_height)
    }
}

impl Generatable for Level {
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let room_number = rng.gen_range(3, 5);

        if self.entrance != (0, 0) {
            self.rooms.push(self.centered_room(self.entrance));
        }

        // Generate rooms
        for _ in self.rooms.len()..room_number {
            self.rooms.push(self.random_room().unwrap());
        }

        // Generate corridors
        for (i, room) in self.rooms.iter().enumerate() {
            // Find the nearest room.
            let next_room = if i == self.rooms.len() - 1 {
                &self.rooms[0]
            } else {
                &self.rooms[i + 1]
            };

            match Corridor::link(room.center, next_room.center) {
                Ok(mut cor) => self.corridors.append(&mut cor),
                Err(e) => println!("{}", e),
            };
        }

        // Create entrance and exit
        if self.entrance == (0, 0) {
            self.entrance = self.rooms[0].center;
        }
        self.exit = self.rooms.last().unwrap().center;

        // Populate the level
        let num_enemies: usize = (self.rooms.len() as f32 * self.depth as f32 * 0.5) as usize;
        for _ in 0..num_enemies {
            // Pick a room
            let mut rng = rand::thread_rng();
            let room = &self.rooms[rng.gen_range(0, self.rooms.len() - 1)];

            // Create the enemy
            self.entities.push(Box::<Character>::new(Enemy::new(
                String::from("snake"),
                2 * self.depth as i32,
                (2.0 * self.depth as f32 * 0.6).round() as i32,
                (20.0 * self.depth as f32 * 0.2).max(80.0).round() as i32,
                0,
                (
                    room.start.0 + rng.gen_range(0, room.width),
                    room.start.1 + rng.gen_range(0, room.height),
                ),
                "s",
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_corridor_detects_horizontal() {
        let start = (0, 0);
        let end = (5, 0);

        let corridor = Corridor::make(start, end).unwrap();
        assert_eq!(corridor, Corridor::new(start, 5, CorridorType::Horizontal));
    }

    #[test]
    fn test_make_corridor_detects_vertical() {
        let start = (0, 0);
        let end = (0, 5);

        let corridor = Corridor::make(start, end).unwrap();
        assert_eq!(corridor, Corridor::new(start, 5, CorridorType::Vertical));
    }

    #[test]
    fn test_make_corridor_with_overlapping_points_should_panic() {
        match Corridor::make((0, 0), (0, 0)) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn test_make_corridor_with_misaligned_points_should_panic() {
        match Corridor::make((3, 3), (5, 5)) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn test_link_corridors_returns_a_vec_of_corridors() {
        let cor = Corridor::link((0, 0), (5, 5)).unwrap();

        let exp_horz = vec![
            Corridor::new((0, 0), 5, CorridorType::Horizontal),
            Corridor::new((5, 0), 5, CorridorType::Vertical),
        ];
        let exp_vert = vec![
            Corridor::new((0, 0), 5, CorridorType::Vertical),
            Corridor::new((0, 5), 5, CorridorType::Horizontal),
        ];

        match cor[0].direction {
            CorridorType::Horizontal => assert_eq!(cor, exp_horz),
            CorridorType::Vertical => assert_eq!(cor, exp_vert),
        };
    }

    #[test]
    fn test_link_corridors_returns_a_vec_of_corridors_on_reversed_diagonal_points() {
        let cor = Corridor::link((5, 5), (0, 0)).unwrap();

        let exp_horz = vec![
            Corridor::new((0, 5), 5, CorridorType::Horizontal),
            Corridor::new((0, 0), 5, CorridorType::Vertical),
        ];
        let exp_vert = vec![
            Corridor::new((5, 0), 5, CorridorType::Vertical),
            Corridor::new((0, 0), 5, CorridorType::Horizontal),
        ];

        match cor[0].direction {
            CorridorType::Horizontal => assert_eq!(cor, exp_horz),
            CorridorType::Vertical => assert_eq!(cor, exp_vert),
        };
    }

    #[test]
    fn test_link_corridors_returns_a_vec_of_corridors_on_reversed_vertical_points() {
        let cor = Corridor::link((0, 5), (5, 0)).unwrap();

        let exp_horz = vec![
            Corridor::new((0, 5), 5, CorridorType::Horizontal),
            Corridor::new((5, 0), 5, CorridorType::Vertical),
        ];
        let exp_vert = vec![
            Corridor::new((0, 0), 5, CorridorType::Vertical),
            Corridor::new((0, 0), 5, CorridorType::Horizontal),
        ];

        match cor[0].direction {
            CorridorType::Horizontal => assert_eq!(cor, exp_horz),
            CorridorType::Vertical => assert_eq!(cor, exp_vert),
        };
    }

    #[test]
    fn test_link_corridors_returns_a_vec_of_corridors_on_reversed_horizontal_points() {
        let cor = Corridor::link((5, 0), (0, 5)).unwrap();

        let exp_horz = vec![
            Corridor::new((0, 0), 5, CorridorType::Horizontal),
            Corridor::new((0, 0), 5, CorridorType::Vertical),
        ];
        let exp_vert = vec![
            Corridor::new((5, 0), 5, CorridorType::Vertical),
            Corridor::new((0, 5), 5, CorridorType::Horizontal),
        ];

        match cor[0].direction {
            CorridorType::Horizontal => assert_eq!(cor, exp_horz),
            CorridorType::Vertical => assert_eq!(cor, exp_vert),
        };
    }

    #[test]
    fn test_link_corridors_with_horizontal_aligned_points_returns_one_corridor() {
        let cor = Corridor::link((0, 0), (5, 0)).unwrap();

        assert_eq!(cor.len(), 1);
        assert_eq!(cor[0], Corridor::new((0, 0), 5, CorridorType::Horizontal));
    }

    #[test]
    fn test_link_corridors_with_vertical_aligned_points_returns_one_corridor() {
        let cor = Corridor::link((0, 0), (0, 5)).unwrap();

        assert_eq!(cor.len(), 1);
        assert_eq!(cor[0], Corridor::new((0, 0), 5, CorridorType::Vertical));
    }
}
