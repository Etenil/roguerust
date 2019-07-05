pub struct Computer {
    level: i32,
    difficulty: i32,
}

pub trait Enemy {
    fn new(level: i32, difficulty: i32) -> Self;

    fn action(&self) -> (i32, i32);

    fn level_up(&mut self);

    fn stats(&self) -> String;
}

impl Enemy for Computer {
    fn new(level: i32, difficulty: i32) -> Computer {
        Computer {
            level: level,
            difficulty: difficulty
        }
    }

    fn action(&self) -> (i32, i32) {
        (self.level, self.difficulty)
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.difficulty += 3;
    }

    fn stats(&self) -> String {
        format!("level: {} difficulty: {}", self.level, self.difficulty)
    }
}
