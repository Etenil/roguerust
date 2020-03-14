mod entities;
mod state;
mod tiling;
mod world;

use std::env;
use std::fs::File;
use std::io::{stdout, Write};

use crossterm::cursor;
use crossterm::execute;
use crossterm::input::{input, InputEvent, KeyEvent};
use crossterm::screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen};
use crossterm::terminal;
use ignore_result::Ignore;
use simplelog::*;

use entities::Player;
use state::State;
use world::{Dungeon, DOWN, LEFT, RIGHT, UP};

fn player_name() -> String {
    match env::var_os("USER") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("Kshar"),
    }
}

fn main() {
    // Set up the debug logger only if required.
    if let Ok(_val) = env::var("DEBUG") {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("roguerust.log").unwrap(),
        )
        .unwrap();
    }

    // Initialise the terminal, the raw alternate mode allows direct character
    // seeking and hides the prompt.
    let term_size = terminal::size().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    let _raw = RawScreen::into_raw_mode();

    // Initialise state, create the player and dungeon
    let mut state = State::new(
        Player::new(player_name(), String::from("Warrior"), 30, 10, 10, 20),
        Dungeon::new(term_size.0 as usize, (term_size.1 - 2) as usize, 5),
    );
    state.init();

    let input = input();
    let mut reader = input.read_sync();

    // Main loop, dispatches events and calls rendering routines. Don't
    // add any game logic here.
    loop {
        state.render_level();
        state.render_entities();
        state.render_player();

        state.render_ui();

        if let Some(event) = reader.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => break,
                InputEvent::Keyboard(KeyEvent::Char('?')) => {
                    state.ui_help();
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

                // Stairs
                InputEvent::Keyboard(KeyEvent::Char('>')) => match state.down_stairs() {
                    Ok(()) => (),
                    Err(info) => state.notify(info),
                },
                InputEvent::Keyboard(KeyEvent::Char('<')) => match state.up_stairs() {
                    Ok(()) => (),
                    Err(info) => state.notify(info),
                },
                _ => (),
            }
        }
        // actors actions (normally attack / interact if on same location as the character)
    }

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
}
