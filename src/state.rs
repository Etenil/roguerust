use pancurses::Window;

use crate::tiling::{TileType, TileGrid, tile_to_str};
use crate::entities::{Character, Entity};
use crate::world::{Dungeon, Generatable, Level};

pub struct State {
    pub player: Character,
    dungeon: Dungeon,
    level: usize,
    grid: Option<TileGrid>
}

pub fn draw_block(window: &Window, block: &TileType) {
    window.printw(tile_to_str(block));
}

impl State {
    pub fn new(player: Character, dungeon: Dungeon) -> State {
        State {
            player,
            dungeon,
            level: 0,
            grid: None
        }
    }

    pub fn init(&mut self) {
        self.dungeon.generate();
        self.player.place(self.current_level().get_start_point());
        self.switch_level(0);
    }

    pub fn switch_level(&mut self, num_level: usize) {
        self.level = num_level;
        self.grid = Some(self.current_level().to_tilegrid().unwrap());
    }

    pub fn render_level(&self, window: &Window) {
        for (linenum, line) in self.grid.as_ref().unwrap().raw_data().iter().enumerate() {
            for block in line.iter() {
                draw_block(&window, &block);
            }
            window.mv(linenum as i32, 0);
        }
    }

    fn render_entity(&self, entity: &dyn Entity, window: &Window) {
        if !entity.is_dirty() {
            return;
        }
        let dirt = entity.get_previous_location();
        window.mv(dirt.1 as i32, dirt.0 as i32);
        draw_block(window, self.grid.as_ref().unwrap().get_block_at(dirt.0, dirt.1));
        window.mv(entity.get_location().1 as i32, entity.get_location().0 as i32);
        draw_block(window, entity.get_tiletype());
    }

    pub fn render_entities(&self, window: &Window) {
        for e in self.current_level().entities.iter() {
            self.render_entity(&**e, window);
        }
    }

    pub fn render_player(&self, window: &Window) {
        self.render_entity(&self.player, window)
    }

    pub fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }

    pub fn get_player_mut(&mut self) -> &mut Character {
        &mut self.player
    }
}