use crossterm::cursor::MoveTo;
use crossterm::{execute, queue, Output};
use std::io::{stdout, Write};

use crate::entities::{Character, Entity, Player};
use crate::tiling::{tile_to_str, Tile, TileGrid, TileType};
use crate::world::{apply_movement, Dungeon, Generatable, Level, Movement};

const PLAYER_SIGHT: usize = 5;

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
        execute!(sout, MoveTo(0, 0)).unwrap();
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
        if !entity.is_visible() || !entity.is_dirty() {
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
            Output(tile_to_str(entity.tile()))
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn render_entities(&self) {
        for e in self.current_level().entities.iter() {
            self.render_entity(&**e);
        }
    }

    pub fn render_player(&mut self) {
        self.render_entity(&self.player);

        self.grid
            .as_mut()
            .unwrap()
            .clear_fog_of_war(self.player.location(), PLAYER_SIGHT);
    }

    fn ui_state_position(&self) -> MoveTo {
        MoveTo(0, (self.dungeon.ysize()) as u16)
    }

    fn ui_notification_position(&self) -> MoveTo {
        MoveTo(0, (self.dungeon.ysize() + 1) as u16)
    }

    pub fn render_ui(&self) {
        let mut sout = stdout();
        queue!(sout, self.ui_state_position(), Output(self.player.stats())).unwrap();
        sout.flush().unwrap();
    }

    pub fn notify(&self, message: String) {
        let mut sout = stdout();
        queue!(
            sout,
            self.ui_notification_position(),
            Output(" ".repeat(self.dungeon.xsize())),
            self.ui_notification_position(),
            Output(message)
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn ui_help(&self) {
        self.notify(String::from(
            "quit: q, movement{up(k), down(j), left(h), right(l)}",
        ))
    }

    pub fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }

    fn can_step_on(tile: &Tile) -> bool {
        match tile.get_type() {
            TileType::Floor => true,
            TileType::StairsDown => true,
            TileType::StairsUp => true,
            _ => false,
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
        self.player.move_by(dir)
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
                self.render_level();
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
                self.render_level();
                Ok(())
            }
            _ => Err(String::from("Not on stairs!")),
        }
    }
}
