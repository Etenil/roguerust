use std::cmp;

use crate::tiling::{Tile, TileType};
use crate::world::{apply_movement, Movement, Point};

pub trait Entity {
    /// Get information about the entity
    fn info(&self) -> String;
    /// Initial placement of the entity
    fn place(&mut self, location: Point);
    /// Get the tiletype for the entity
    fn tile(&self) -> &Tile;
    /// Get the entity's current location
    fn location(&self) -> &Point;
    /// Get the entity's previous location (before it moved)
    fn previous_location(&self) -> &Point;
    /// Move the entity to another point
    fn move_to(&mut self, location: Point);
    /// Move the entity with a movement differential
    fn move_by(&mut self, movement: Movement) -> Result<(), String>;
    /// Know if the entity needs to be re-rendered
    fn is_dirty(&self) -> bool;
    /// Declare the entity clean
    fn clean(&mut self);
    fn visibility(&mut self, visible: bool);
    fn is_visible(&self) -> bool;
}

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub health: i32,
    pub level: i32,
    location: Point,
    previous_location: Point,
    dirty: bool,
    max_health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
    xp: i32,
    tile: Tile,
}

pub trait Enemy {
    fn new(
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        location: Point,
        tile_str: &'static str,
    ) -> Self;

    fn set_tile(&mut self, tile: Tile);
}

pub trait Player {
    fn new(name: String, class: String, health: i32, attack: i32, dodge: i32, luck: i32) -> Self;
    fn damage(&mut self, damage_amount: i32);
    fn heal(&mut self, heal_amount: i32);
    fn attack(&self) -> i32;
    fn dodge(&self) -> i32;
    fn stats(&self) -> String;
}

impl Entity for Character {
    fn place(&mut self, location: Point) {
        self.location = location;
        self.previous_location = location;
        self.dirty = true;
    }

    fn info(&self) -> String {
        format!(
            "{} \thp: {} attack: {} dodge: {} luck: {}",
            self.class, self.health, self.attack, self.dodge, self.luck
        )
    }

    fn tile(&self) -> &Tile {
        &self.tile
    }

    fn location(&self) -> &Point {
        &self.location
    }

    fn previous_location(&self) -> &Point {
        &self.previous_location
    }

    fn move_to(&mut self, location: Point) {
        self.previous_location = self.location;
        self.location = location;
        self.dirty = true;
    }

    fn move_by(&mut self, movement: Movement) -> Result<(), String> {
        self.previous_location = self.location;
        self.location = apply_movement(self.location, movement)?;
        Ok(())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn clean(&mut self) {
        self.dirty = false;
    }

    fn visibility(&mut self, visible: bool) {
        if visible != self.is_visible() {
            self.dirty = true;
        }
        self.tile.visibility(visible)
    }

    fn is_visible(&self) -> bool {
        self.tile.is_visible()
    }
}

impl Enemy for Character {
    fn new(
        class: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
        location: Point,
        tile_str: &'static str,
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
            location,
            previous_location: location,
            tile: Tile::from(TileType::Character(tile_str)),
            dirty: false,
        }
    }

    fn set_tile(&mut self, tile: Tile) {
        self.tile = tile
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
    ) -> Character {
        Character {
            name,
            class,
            max_health: health,
            health,
            attack,
            dodge,
            luck,
            xp: 0,
            level: 0,
            location: (0, 0),
            previous_location: (0, 0),
            tile: Tile::new(
                TileType::Player,
                true, // player is visible by default
                false,
		true,
		false
            ),
            dirty: false,
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
            "{}({}) - hp: {}/{} attack: {} dodge: {} luck: {} experience: {}",
            self.name,
            self.class,
            self.health,
            self.max_health,
            self.attack,
            self.dodge,
            self.luck,
            self.xp
        )
    }
}
