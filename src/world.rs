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
    fn make_corridor(&mut self, start: (usize, usize), end: (usize, usize)) {
        for x in start.0..end.0 {
            for y in start.1..end.1 {
                self.world[x - 1][y] = BlockType::Wall;
                self.world[x][y] = BlockType::Floor;
                self.world[x + 1][y] = BlockType::Wall;
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

        self.make_corridor((1, 10), (1, 13))
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
