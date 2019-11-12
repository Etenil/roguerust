use std::cmp;

use pancurses::{Window};
use crate::world::{Point};
use crate::tiling::TileType;

pub trait Entity {
    fn info(&self) -> String;
    fn place(&mut self, location: Point);
    fn get_tiletype(&self) -> &TileType;
    fn get_location(&self) -> &Point;
}

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub health: i32,
    pub level: i32,
    pub location: Point,
    max_health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
    xp: i32,
    tile_type: TileType
}

pub trait Enemy {
    fn new(
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        location: Point
    ) -> Self;

    fn set_tile_type(&mut self, tile_type: TileType);
}

pub trait Player {
    fn new(
        name: String,
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        level: i32
    ) -> Self;
    fn damage(&mut self, damage_amount: i32);
    fn heal(&mut self, heal_amount: i32);
    fn attack(&self) -> i32;
    fn dodge(&self) -> i32;
    fn stats(&self) -> String;
}

impl Entity for Character {
    fn place(&mut self, location: Point) {
        self.location = location;
    }

    fn info(&self) -> String {
        format!(
            "{} \thp: {} attack: {} dodge: {} luck: {}",
            self.class, self.health, self.attack, self.dodge, self.luck
        )
    }

    fn get_tiletype(&self) -> &TileType {
        &self.tile_type
    }

    fn get_location(&self) -> &Point {
        &self.location
    }
}

impl Enemy for Character {
    fn new(
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        location: Point
    ) -> Character {
        Character {
            name: class.clone(),
            class: class.clone(),
            max_health: health,
            health,
            attack,
            dodge,
            luck,
            level: 0,
            xp: 0,
            location: location,
            tile_type: TileType::Character
        }
    }

    fn set_tile_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type
    }
}

impl Player for Character {
    fn new(
        name: String,
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        level: i32
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
            level: 0,
            location: (0, 0),
            tile_type: TileType::Player
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

    fn stats(&self) -> String {
        format!(
            "{} - hp: {}/{} attack: {} dodge: {} luck: {} experience: {}",
            self.class, self.health, self.max_health, self.attack, self.dodge, self.luck, self.xp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_attack() {
        let bob: Character = Enemy::new("Rogue".to_string(), 1, 4, 1, 4, (0, 0));

        assert_eq!(bob.attack(), 6);
    }
}
