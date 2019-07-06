pub enum BlockType {
    Nothing,
    Wall,
    Floor,
}

pub struct World {
    size: usize,
    world: Vec<Vec<BlockType>>
}

pub trait GameWorld<'a> {
    fn new(size: usize) -> Self;

    fn generate(&mut self);

    fn get_world(&'a self) -> &'a Vec<Vec<BlockType>>;

    fn get_item(&'a self, x: usize, y: usize) -> &'a BlockType;
}

impl World {
    fn make_vertical_corridor(&mut self, start: (usize, usize), length: usize) {
        let x = start.0;
        let endy = start.1 + length;
        for y in start.1..endy {
            self.wall_up(x - 1, y);
            self.set_tile(x, y, BlockType::Floor);
            self.wall_up(x + 1, y);
        }
    }

    fn make_horizontal_corridor(&mut self, start: (usize, usize), length: usize) {
        let y = start.1;
        let endx = start.0 + length;
        for x in start.0..endx {
            self.wall_up(x, y - 1);
            self.set_tile(x, y, BlockType::Floor);
            self.wall_up(x, y - 1);
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, block: BlockType) {
        self.world[y][x] = block;
    }

    /// Puts a wall on the coordinates if it isn't a floor.
    fn wall_up(&mut self, x: usize, y: usize) {
        self.set_tile(x, y, match self.world[y][x] {
            BlockType::Floor => BlockType::Floor,
            _ => BlockType::Wall
        })
    }

    /// Creates a room at the given coordinates of the given size.
    fn make_room(&mut self, start: (usize, usize), width: usize, height: usize) {
        let endx = start.0 + width;
        let endy = start.1 + height;

        // Draw the walls
        for x in start.0..endx {
            self.wall_up(x, start.1);
            self.wall_up(x, endy);
        }

        for y in start.1..endy {
            self.wall_up(start.0, y);
            self.wall_up(endx, y);
        }

        // Fill the room
        for x in (start.0 + 1)..(endx) {
            for y in (start.1 + 1)..(endy) {
                self.set_tile(x, y, BlockType::Floor);
            }
        }
    }
}

impl<'a> GameWorld<'a> for World {
    fn new(size: usize) -> World {
        World {
            size: size,
            world: Vec::with_capacity(size)
        }
    }
    
    fn generate(&mut self) {
        for _ in 0..self.size {
            let mut subvec = Vec::with_capacity(self.size);
            for _ in 0..self.size {
                subvec.push(BlockType::Nothing);
            }
            self.world.push(subvec);
        }

        self.make_room((1, 13), 5, 7);
        self.make_vertical_corridor((3, 5), 10);
    }

    fn get_world(&'a self) -> &'a Vec<Vec<BlockType>> {
        &self.world
    }

    fn get_item(&'a self, x: usize, y: usize) -> &'a BlockType {
        &self.world[x][y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_world() {
        let mut world = World::new(128);
        world.generate();

        assert_eq!(world.get_world().len(), 128);
        assert_eq!(world.get_world()[0].len(), 128);
        match world.get_world()[0][0] {
            BlockType::Nothing => assert!(true),
            _ => assert!(false)
        }
    }
}
