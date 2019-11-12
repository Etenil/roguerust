extern crate rand;
extern crate pancurses;

#[macro_use]
extern crate text_io;

mod state;
mod entities;
mod world;
mod tiling;

use entities::Player;
use pancurses::{
    initscr,
    endwin,
    Input,
    noecho
};
use state::State;
use world::{Dungeon};


fn main() {
    let window = initscr();

    let mut state = State::new(
        Player::new(
            "Kshar".to_string(),
            "Warrior".to_string(),
            30,
            10,
            10,
            20,
            1,
            (0, 0)
        ),
        Dungeon::new(window.get_max_x() as usize, window.get_max_y() as usize, 5),
    );

    state.init();

    // Dump the whole dungeon structure in terminal for debugging
    state.debug();

    window.keypad(true);
    noecho();
    
    loop {
        // update actors
        
        state.render_level(&window);

        // update character
        state.show_character(&window);
        
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
        // actors actions (normally attack / interact if on same location as the character)
    }
    endwin();
}
