extern crate crossterm;
extern crate ignore_result;
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
use entities::Player;
use ignore_result::Ignore;
use state::State;
use std::env;
use std::io::{stdout, Write};
use world::{Dungeon, DOWN, LEFT, RIGHT, UP};

fn player_name() -> String {
    match env::var_os("USER") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("Kshar"),
    }
}

fn main() {
    let term_size = terminal::size().unwrap();

    let mut state = State::new(
        Player::new(player_name(), String::from("Warrior"), 30, 10, 10, 20),
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

        state.render_ui();

        if let Some(event) = reader.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => break,
                InputEvent::Keyboard(KeyEvent::Char('?')) => {
                    execute!(stdout(), Output("q: quit")).unwrap()
                }
                InputEvent::Keyboard(KeyEvent::Char('j')) => state.move_player(DOWN).ignore(),
                InputEvent::Keyboard(KeyEvent::Char('k')) => state.move_player(UP).ignore(),
                InputEvent::Keyboard(KeyEvent::Char('h')) => state.move_player(LEFT).ignore(),
                InputEvent::Keyboard(KeyEvent::Char('l')) => state.move_player(RIGHT).ignore(),
                // Arrow keys for noobs
                InputEvent::Keyboard(KeyEvent::Down) => state.move_player(DOWN).ignore(),
                InputEvent::Keyboard(KeyEvent::Up) => state.move_player(UP).ignore(),
                InputEvent::Keyboard(KeyEvent::Left) => state.move_player(LEFT).ignore(),
                InputEvent::Keyboard(KeyEvent::Right) => state.move_player(RIGHT).ignore(),
                _ => (),
            }
        }
        // actors actions (normally attack / interact if on same location as the character)
    }

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
}
