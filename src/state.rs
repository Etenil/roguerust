use pancurses::Window;
use std::env;

use crate::entities::{Character, Entity};
use crate::world::{Dungeon, Generatable, Level};

pub struct State {
    pub player: Character,
    pub dungeon: Dungeon,
    pub level: usize,
}

impl State {
    pub fn new(
        player: Character,
        dungeon: Dungeon,
    ) -> Self {
        State {
            player: player,
            dungeon: dungeon,
            level: 0,
        }
    }

    pub fn init(&mut self) {
        self.dungeon.generate();
        self.player.place(self.current_level().get_start_point());
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

    fn current_level(&self) -> &Level {
        &self.dungeon.levels[self.level]
    }
}