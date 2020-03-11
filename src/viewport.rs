use crossterm::cursor::MoveTo;
use crossterm::{execute, queue, Output};
use log::debug;
use std::io::{stdout, Write};

use crate::entities::{Character, Entity, Player};
use crate::state::State;
use crate::tiling::{tile_to_str, Tile, TileGrid, TileType};
use crate::world::{apply_movement, Dungeon, Generatable, Level, Movement};

pub trait ViewPort {
    fn render_state(&mut self, state: &State);
}

pub struct CrossTermViewPort {
    xsize: usize,
    ysize: usize,
    // Use below when switching to moveable window
    //start: (usize, usize)
}

impl CrossTermViewPort {
    pub fn new(xsize: usize, ysize: usize) -> CrossTermViewPort {
        CrossTermViewPort { xsize, ysize }
    }

    fn draw_level(&self, state: &State) {
        let mut sout = stdout();
        let grid = state.get_grid().unwrap();
        execute!(sout, MoveTo(0, 0)).unwrap();
        for (linenum, line) in grid.raw_data().iter().enumerate() {
            debug!("Drawing linenum {} -- {:?}", linenum, line);
            let linestr = line.iter().map(tile_to_str).collect::<Vec<&str>>();
            let mut linestr2 = String::from("");
            for chr in linestr {
                linestr2.push_str(chr);
            }
            queue!(sout, Output(linestr2), MoveTo(0, linenum as u16)).unwrap();
            sout.flush().unwrap();
        }
    }

    fn draw_entity(&self, state: &State, entity: &dyn Entity) {
        if !entity.is_visible() || !entity.is_dirty() {
            return;
        }
        let grid = state.get_grid().unwrap();
        let dirt = entity.previous_location();
        let background = grid.block_at(dirt.0, dirt.1);
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

    fn draw_entities(&self, state: &State) {
        for e in state.current_level().entities.iter() {
            self.draw_entity(state, &**e);
        }
    }

    fn draw_player(&mut self, state: &State) {
        self.draw_entity(state, state.get_player());
    }

    fn ui_state_position(&self) -> MoveTo {
        MoveTo(0, (self.ysize) as u16)
    }

    fn ui_notification_position(&self) -> MoveTo {
        MoveTo(0, (self.ysize + 1) as u16)
    }

    fn draw_ui(&self, state: &State) {
        let mut sout = stdout();
        queue!(
            sout,
            self.ui_state_position(),
            Output(state.get_player().stats())
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn notify(&self, message: String) {
        let mut sout = stdout();
        queue!(
            sout,
            self.ui_notification_position(),
            Output(" ".repeat(self.xsize)),
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

impl ViewPort for CrossTermViewPort {
    fn render_state(&mut self, state: &State) {
        self.draw_level(state);
        self.draw_entities(state);
        self.draw_player(state);
        self.draw_ui(state);
    }
}
