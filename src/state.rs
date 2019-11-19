use crossterm::cursor::MoveTo;
use crossterm::{queue, Output};
use std::io::{stdout, Write};

use crate::entities::{Character, Entity, Player};
use crate::tiling::{tile_to_str, TileGrid, TileType};
use crate::world::{apply_movement, Dungeon, Generatable, Level, Movement};

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
    }

    pub fn switch_level(&mut self, num_level: usize) {
        self.level = num_level;
        self.grid = Some(self.current_level().to_tilegrid().unwrap());
    }

    pub fn render_level(&self) {
        let mut sout = stdout();
        for (linenum, line) in self.grid.as_ref().unwrap().raw_data().iter().enumerate() {
            let linestr = line.iter().map(tile_to_str).collect::<Vec<&str>>();
            let mut linestr2 = String::from("");
            for chr in linestr {
                linestr2.push_str(chr);
            }
            queue!(sout, Output(linestr2), MoveTo(0, linenum as u16)).unwrap();
            sout.flush().unwrap();
        }
    }

    fn render_entity(&self, entity: &dyn Entity) {
        if !entity.is_dirty() {
            return;
        }
        let dirt = entity.previous_location();
        let background = self.grid.as_ref().unwrap().block_at(dirt.0, dirt.1);
        let mut sout = stdout();
        queue!(
            sout,
            MoveTo(dirt.0 as u16, dirt.1 as u16),
            Output(tile_to_str(background)),
            MoveTo(entity.location().0 as u16, entity.location().1 as u16),
            Output(tile_to_str(entity.tiletype()))
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn render_entities(&self) {
        for e in self.current_level().entities.iter() {
            self.render_entity(&**e);
        }
    }

    pub fn render_player(&self) {
        self.render_entity(&self.player)
    }

    pub fn render_ui(&self) {
        let mut sout = stdout();
        queue!(
            sout,
            MoveTo(0, (self.dungeon.ysize() + 1) as u16),
            Output(self.player.stats())
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }

    fn can_step_on(tile: &TileType) -> bool {
        match tile {
            TileType::Floor => true,
            TileType::StairsDown => true,
            TileType::StairsUp => true,
            _ => false,
        }
    }

    pub fn move_player(&mut self, dir: Movement) -> Result<(), String> {
        match &self.grid {
            None => Err(String::from("No level loaded!")),
            Some(grid) => {
                let loc = apply_movement(*self.player.location(), dir)?;
                // Is the new location colliding with anything?
                if !State::can_step_on(grid.block_at(loc.0, loc.1)) {
                    return Err(String::from("Can't move entity!"));
                }
                self.player.move_by(dir)
            }
        }
    }
}
