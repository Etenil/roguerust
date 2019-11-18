extern crate crossterm;
extern crate rand;
extern crate text_io;

mod entities;
mod state;
mod tiling;
mod world;

use crossterm::cursor;
use crossterm::input::{input, InputEvent, KeyEvent};
use crossterm::screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen};
use crossterm::terminal;
use crossterm::{execute, Output};
use entities::{Entity, Player};
use state::State;
use std::env;
use std::io::{stdout, Write};
use world::{Dungeon, DOWN, LEFT, RIGHT, UP};

fn get_player_name() -> String {
    match env::var_os("USER") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("Kshar"),
    }
}

fn main() {
    let term_size = terminal::size().unwrap();

    let mut state = State::new(
        Player::new(get_player_name(), String::from("Warrior"), 30, 10, 10, 20),
        Dungeon::new(term_size.0 as usize, (term_size.1 - 2) as usize, 5),
    );

    state.init();

    execute!(stdout(), EnterAlternateScreen).unwrap();
    execute!(stdout(), cursor::Hide).unwrap();

    let _raw = RawScreen::into_raw_mode();

    state.render_level();

    let input = input();
    let mut reader = input.read_sync();

    loop {
        // update
        state.render_entities();

        state.render_player();

        if let Some(event) = reader.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => break,
                InputEvent::Keyboard(KeyEvent::Char('?')) => {
                    execute!(stdout(), Output("q: quit")).unwrap()
                }
                InputEvent::Keyboard(KeyEvent::Char('j')) => state.player.move_by(DOWN).unwrap(),
                InputEvent::Keyboard(KeyEvent::Char('k')) => state.player.move_by(UP).unwrap(),
                InputEvent::Keyboard(KeyEvent::Char('h')) => state.player.move_by(LEFT).unwrap(),
                InputEvent::Keyboard(KeyEvent::Char('l')) => state.player.move_by(RIGHT).unwrap(),
                // Arrow keys for noobs
                InputEvent::Keyboard(KeyEvent::Down) => state.player.move_by(DOWN).unwrap(),
                InputEvent::Keyboard(KeyEvent::Up) => state.player.move_by(UP).unwrap(),
                InputEvent::Keyboard(KeyEvent::Left) => state.player.move_by(LEFT).unwrap(),
                InputEvent::Keyboard(KeyEvent::Right) => state.player.move_by(RIGHT).unwrap(),
                _ => (),
            }
        }
        // actors actions (normally attack / interact if on same location as the character)
    }

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
}
