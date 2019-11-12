extern crate rand;
extern crate pancurses;

#[macro_use]
extern crate text_io;

mod character;
mod computer;
mod world;

use character::Player;
use computer::Enemy;
use pancurses::{Window, initscr, endwin};
use world::{Dungeon, Level, Generable, TileType};

fn tile_to_str(tile: &TileType) -> &str {
    match tile {
        TileType::Floor => ".",
        TileType::Wall => "█",
        TileType::Empty => " ",
        TileType::StairsDown => ">",
        TileType::StairsUp => "<",
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

fn debug_level(level: &Level) {
    let grid = level.to_tilegrid().unwrap();

    for line in grid.raw_data().iter() {
        for block in line.iter() {
            print!("{}", tile_to_str(block));
        }
        print!("\n");
    }
}

fn main() {
    let mut level = 0;
    let mut dungeon = Dungeon::new(80, 24, 5);
    dungeon.generate();

    for l in dungeon.levels {
        debug_level(&l);
    }

    // let window = initscr();

    // render_dungeon(&window, &world);

    // window.refresh();

    // window.getch();
    // endwin();
}
