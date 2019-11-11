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

fn draw_block(window: &Window, block: &TileType) {
    let repr = match block {
        TileType::Floor => ".",
        TileType::Wall => "█",
        TileType::Corridor => "#",
        TileType::Empty => " "
    };

    window.printw(repr);
}

fn render_world(window: &Window, world: &World) {
    let grid = world.to_tilegrid();

    for (linenum, line) in grid.raw_data().iter().enumerate() {
        for block in line.iter() {
            draw_block(&window, block);
        }
        window.mv(linenum as i32, 0);
    }
}

fn debug_world(world: &World) {
    let grid = world.to_tilegrid();

    for (line) in grid.raw_data().iter() {
        for block in line.iter() {
            print!("{}", match block {
                TileType::Floor => ".",
                TileType::Wall => "█",
                TileType::Corridor => "#",
                TileType::Empty => " ",
                _ => "?"
            });
        }
        print!("\n");
    }
}

fn main() {
    let mut world = World::new(24);
    world.generate();

    debug_world(&world);

    // let window = initscr();

    // render_world(&window, &world);

    // window.refresh();

    // window.getch();
    // endwin();
}
