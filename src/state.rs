use pancurses::Window;
use std::env;

use crate::entities::{Character, Entity};
use crate::world::{Dungeon, Generatable, Level};

pub struct State {
    pub character: Character,
    pub dungeon: Dungeon,
    pub level: usize,
}

impl State {
    pub fn new(
        character: Character,
        dungeon: Dungeon,
    ) -> State {
        State {
            character: character,
            dungeon: dungeon,
            level: 0,
        }
    }

    pub fn init(&mut self) {
        self.dungeon.generate();
        self.character.place(self.current_level().get_start_point());
    }

    pub fn debug(&self) {
        match env::var("DEBUG") {
            Ok(_) => {
                self.dungeon.debug_levels();
            },
            Err(_) => ()
        };
    }

    pub fn render_level(&self, window: &Window) {
        self.current_level().render(window);
    }

    pub fn show_character(&self, window: &Window) {
        self.character.render(window);
        
    //     window.mv(window.get_max_y() - 2, 0);
    //     window.clrtoeol();
        
    //     window.refresh();

    //     window.addstr(self.character.info() + "\n");

    //     window.mv(self.character.location.1 as i32,self.character.location.0 as i32);
    //     window.refresh();
    //     draw_block(&window, self.character.tile_type);
    //     window.refresh();
    }

    fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }
}