extern crate rand;
extern crate pancurses;

#[macro_use]
extern crate text_io;

mod character;
mod computer;
mod state;
mod world;

use character::Player;
use character::Character;
use pancurses::{
    initscr,
    endwin,
    Input
};
use state::State;
use world::Dungeon;


fn main() {
    let mut state = State::new(
        Character::new(
            "Kshar".to_string(),
            "Warror".to_string(),
            30,
            10,
            10,
            20,
        ),
        Dungeon::new(80, 24, 5),
    );

    state.init();

    // Dump the whole dungeon structure in terminal for debugging
    state.debug();

    let window = initscr();

    state.render_level(&window);

    loop {
        // update actors
        // update character
        window.refresh();

        // get input and execute it
        match window.getch() {

            Some(Input::Character('h')) => { window.addstr("q: quit\n"); },
            // Some(Input::KeyDown) => { window.addstr("down\n"); },
            // Some(Input::KeyUp) => { window.addch('b'); },
            // Some(Input::KeyLeft) => { window.addch('c'); },
            // Some(Input::KeyRight) => { window.addch('d'); },
            Some(Input::Character('q')) => break,
            Some(_) => (),
            None => (),
        }
    }
    endwin();

    println!("You quit with {} gold pieces", state.character.get_gold())
}
