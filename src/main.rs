mod entities;
mod events;
mod state;
mod tiling;
mod viewport;
mod world;

use ignore_result::Ignore;
use simplelog::*;
use std::env;
use std::fs::File;

use entities::Player;
use events::ViewportEvent;
use state::State;
use viewport::{CrossTermViewPort, ViewPort};
use world::Dungeon;

const DUNGEON_SIZE_X: usize = 20;
const DUNGEON_SIZE_Y: usize = 20;
const DUNGEON_DEPTH: usize = 5;

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

    let mut state = State::new(
        Player::new(player_name(), String::from("Warrior"), 30, 10, 10, 20),
        Dungeon::new(DUNGEON_SIZE_X, DUNGEON_SIZE_Y, DUNGEON_DEPTH),
    );
    let mut window = CrossTermViewPort::new();
    state.init();

    // Main loop, dispatches events and calls rendering routines. Don't
    // add any game logic here.
    loop {
        window.render_state(&state);

        if let Some(event) = window.wait_input() {
            match event {
                ViewportEvent::Quit => break,
                ViewportEvent::MovePlayer(direction) => state.move_player(direction).ignore(),
                ViewportEvent::DownStairs => match state.down_stairs() {
                    Ok(()) => (),
                    Err(info) => window.notify(info),
                },
                ViewportEvent::UpStairs => match state.up_stairs() {
                    Ok(()) => (),
                    Err(info) => window.notify(info),
                },
                _ => (),
            }
        }

        // actors actions (normally attack / interact if on same location as the character)
    }
}
