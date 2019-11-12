extern crate rand;
extern crate pancurses;

#[macro_use]
extern crate text_io;

mod entities;
mod world;
mod tiling;

use entities::{Character, Player, Entity};
use pancurses::{Window, initscr, endwin, Input, noecho};
use world::{Dungeon, Level, Generatable};
use tiling::TileType;

fn tile_to_str(tile: &TileType) -> &str {
    match tile {
        TileType::Floor => ".",
        TileType::Wall => "#",
        TileType::Empty => " ",
        TileType::StairsDown => ">",
        TileType::StairsUp => "<",
        TileType::Character => "@",
        _ => "?"
    }
}

fn draw_block(window: &Window, block: &TileType) {
    window.printw(tile_to_str(block));
}

fn render_level(window: &Window, level: &Level) {
    let grid = level.to_tilegrid().unwrap();

    for (linenum, line) in grid.raw_data().iter().enumerate() {
        for block in line.iter() {
            draw_block(&window, block);
        }
        window.mv(linenum as i32, 0);
    }
}

fn main() {
    let window = initscr();
    let mut level = 0;

    let mut dungeon = Dungeon::new(
        window.get_max_x() as usize,
        window.get_max_y() as usize - 2,
        5
    );
    dungeon.generate();

    let start_location = dungeon.levels[0].get_start_point();

    let mut character: Character = Character::new(
        "Kshar".to_string(),
        "Warror".to_string(),
        30,
        10,
        10,
        20,
        start_location
    );
    character.place(start_location);

    render_level(&window, &dungeon.levels[0]);

    window.keypad(true);
    noecho();

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
}
