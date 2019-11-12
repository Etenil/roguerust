use pancurses::Window;
use std::env;

use crate::entities::{Character, Entity, Render};
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

    pub fn render_level(&self, window: &Window) {
        self.current_level().render(window);
    }

    pub fn show_character(&self, window: &Window) {
        self.character.render(window);
    }

    fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }
}