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
use rand::Rng;
use std::io;
use std::convert::TryFrom;
use world::{World, GameWorld, TileType};

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

fn render_world(window: &Window, world: &World) {
    let grid = world.to_tilegrid().unwrap();

    for (linenum, line) in grid.raw_data().iter().enumerate() {
        for block in line.iter() {
            draw_block(&window, block);
        }
        window.mv(linenum as i32, 0);
    }
}

fn debug_world(world: &World) {
    let grid = world.to_tilegrid().unwrap();

    for line in grid.raw_data().iter() {
        for block in line.iter() {
            print!("{}", tile_to_str(block));
        }
        print!("\n");
    }
}

fn main() {
    let mut world = World::new(80, 24);
    world.generate();

    debug_world(&world);

    // let window = initscr();

    // render_world(&window, &world);

    // window.refresh();

    // window.getch();
    // endwin();
}
