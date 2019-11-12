use pancurses::Window;

use crate::tiling::{TileType, tile_to_str};
use crate::entities::{Character, Entity};
use crate::world::{Dungeon, Generatable, Level};

pub struct State {
    pub player: Character,
    pub dungeon: Dungeon,
    pub level: usize,
}

pub fn draw_block(window: &Window, block: &TileType) {
    window.printw(tile_to_str(block));
}

impl State {
    pub fn new(
        player: Character,
        dungeon: Dungeon,
    ) -> State {
        State {
            player,
            dungeon,
            level: 0,
        }
    }

    pub fn init(&mut self) {
        self.dungeon.generate();
        self.player.place(self.current_level().get_start_point());
    }

    pub fn render_level(&self, window: &Window) {
        let grid = self.current_level().to_tilegrid().unwrap();

        for (linenum, line) in grid.raw_data().iter().enumerate() {
            for block in line.iter() {
                draw_block(&window, &block);
            }
            window.mv(linenum as i32, 0);
        }
    }

    fn render_entity(&self, entity: &dyn Entity, window: &Window) {
        window.mv(entity.get_location().1 as i32, entity.get_location().0 as i32);
        draw_block(window, entity.get_tiletype());
    }

    pub fn render_entities(&self, window: &Window) {
        // TODO
    }

    pub fn render_player(&self, window: &Window) {
        self.render_entity(&self.player, window)
    }

    fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }
}