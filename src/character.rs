use std::cmp;

use crate::world::Point;

pub struct Character {
    pub name: String,
    pub class: String,
    pub health: i32,
    max_health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
    xp: i32,
    gold: i32,
    location: Point
}

pub trait Player {
    fn new(
        name: String,
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
    ) -> Character;

    fn place(&mut self, location: Point);

    fn select(&self, player_name: String, player_luck: i32) -> Self;

    fn damage(&mut self, damage_amount: i32);

    fn heal(&mut self, heal_amount: i32);

    fn attack(&self) -> i32;

    fn dodge(&self) -> i32;

    fn info(&self) -> String;

    fn stats(&self) -> String;

    fn walk(&mut self, dir: Direction);

    fn get_gold(&self) -> i32;
}

pub enum Direction {
    North,
    South,
    East,
    West
}

impl Player for Character {
    fn new(
        name: String,
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
    ) -> Character {
        Character {
            name: name,
            class: class,
            max_health: health,
            health: health,
            attack: attack,
            dodge: dodge,
            luck: luck,
            xp: 0,
            gold: 0,
            location: (0, 0),
        }
    }

    fn place(&mut self, location: Point) {
        self.location = location;
    }

    fn select(&self, player_name: String, player_luck: i32) -> Self {
        Self::new(
            player_name,
            self.class.to_string(),
            self.health,
            self.attack,
            self.dodge,
            self.luck + player_luck,
        )
    }

    fn walk(&mut self, dir: Direction) {
        match dir {
            Direction::North => { (); },
            Direction::South => { (); },
            Direction::East => { (); },
            Direction::West => { (); }
        }
    }

    fn damage(&mut self, damage_amount: i32) {
        self.health = cmp::max(0, self.health - damage_amount);
        self.xp += 2;
    }

    fn heal(&mut self, heal_amount: i32) {
        if (self.health) <= self.max_health {
            self.health = cmp::min(self.health + heal_amount, self.max_health);
            self.xp += 1;
        }
    }

    fn attack(&self) -> i32 {
        self.xp + self.attack + self.luck / 2
    }

    fn dodge(&self) -> i32 {
        self.xp + self.dodge + self.luck / 2
    }

    fn info(&self) -> String {
        format!(
            "{} \thp: {} attack: {} dodge: {} luck: {}",
            self.class, self.health, self.attack, self.dodge, self.luck
        )
    }

    fn stats(&self) -> String {
        format!(
            "{} - hp: {}/{} attack: {} dodge: {} luck: {} experience: {} gold: {}",
            self.class, self.health, self.max_health, self.attack, self.dodge, self.luck, self.xp, self.gold
        )
    }

    fn get_gold(&self) -> i32 {
        self.gold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_attack() {
        let player = Character::new("".to_string(), "Rogue".to_string(), 1, 4, 1, 4);

        assert_eq!(player.attack(), 6);
    }
}
