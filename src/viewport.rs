use crossterm::cursor::MoveTo;
use crossterm::{execute, queue, Output};
use std::io::{stdout, Write};

use crate::entities::{Character, Entity, Player};
use crate::state::State;
use crate::tiling::{tile_to_str, Tile, TileGrid, TileType};
use crate::world::{apply_movement, Dungeon, Generatable, Level, Movement};


pub trait ViewPort {
    fn render_state(&mut self, &State);
}

pub struct CrossTermViewPort {
    xsize: usize,
    ysize: usize
}

impl CrossTermViewPort {
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
}
