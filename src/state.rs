use crate::entities::{Character, Entity};
use crate::tiling::{Tile, TileGrid, TileType};
use crate::world::{apply_movement, Dungeon, Generatable, Level, Movement};

const PLAYER_SIGHT: usize = 3;

pub struct State {
    pub player: Character,
    dungeon: Dungeon,
    level: usize,
    grid: Option<TileGrid>,
}

impl State {
    pub fn new(player: Character, dungeon: Dungeon) -> State {
        State {
            player,
            dungeon,
            level: 0,
            grid: None,
        }
    }

    pub fn init(&mut self) {
        self.dungeon.generate();
        self.switch_level(0);
        self.player.place(self.current_level().start_point());
        self.fog_of_war();
    }

    pub fn get_grid(&self) -> Option<&TileGrid> {
        self.grid.as_ref()
    }

    pub fn get_player(&self) -> &Character {
        &self.player
    }

    pub fn switch_level(&mut self, num_level: usize) {
        self.level = num_level;
        self.grid = Some(self.current_level().to_tilegrid().unwrap());
        self.fog_of_war();
    }

    pub fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }

    pub fn current_level_mut(&mut self) -> &mut Level {
        &mut self.dungeon.levels[self.level]
    }

    fn can_step_on(tile: &Tile) -> bool {
        match tile.get_type() {
            TileType::Floor => true,
            TileType::StairsDown => true,
            TileType::StairsUp => true,
            _ => false,
        }
    }

    pub fn fog_of_war(&mut self) {
        {
            let grid = self.grid.as_mut().unwrap();
            grid.clear_fog_of_war(self.player.location(), PLAYER_SIGHT);
        }

        for i in 0..self.current_level().entities.len() {
            let loc = *self.current_level().entities[i].location();
            if self
                .grid
                .as_ref()
                .unwrap()
                .block_at(loc.0, loc.1)
                .is_visible()
                && !self.current_level().entities[i].is_visible()
            {
                self.current_level_mut().entities[i].visibility(true);
            }
        }
    }

    pub fn move_player(&mut self, dir: Movement) -> Result<(), String> {
        let grid = match &self.grid {
            Some(g) => g,
            None => return Err(String::from("No level loaded!")),
        };

        let loc = apply_movement(*self.player.location(), dir)?;
        // Is the new location colliding with anything?
        if !State::can_step_on(grid.block_at(loc.0, loc.1)) {
            return Err(String::from("Can't move entity!"));
        }
        let ret = self.player.move_by(dir);
        self.fog_of_war();
        ret
    }

    pub fn down_stairs(&mut self) -> Result<(), String> {
        let grid = match &self.grid {
            Some(g) => g,
            None => return Err(String::from("No level loaded!")),
        };

        if self.level == self.dungeon.depth() - 1 {
            return Err(String::from("Already at the bottom level"));
        }

        let loc = self.player.location();
        match grid.block_at(loc.0, loc.1).get_type() {
            TileType::StairsDown => {
                self.switch_level(self.level + 1);
                Ok(())
            }
            _ => Err(String::from("Not on stairs!")),
        }
    }

    pub fn up_stairs(&mut self) -> Result<(), String> {
        let grid = match &self.grid {
            Some(g) => g,
            None => return Err(String::from("No level loaded!")),
        };

        if self.level == 0 {
            return Err(String::from("Already at the top level"));
        }

        let loc = self.player.location();
        match grid.block_at(loc.0, loc.1).get_type() {
            TileType::StairsUp => {
                self.switch_level(self.level - 1);
                Ok(())
            }
            _ => Err(String::from("Not on stairs!")),
        }
    }
}
