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
use world::{World, GameWorld, BlockType};

fn draw_block(window: &Window, block: &BlockType) {
    let repr = match block {
        BlockType::Floor => ".",
        BlockType::Wall => "0",
        BlockType::Nothing => " "
    };

    print!("{}", repr);

    window.printw(repr);
}

fn render_world(window: &Window, world: &World) {
    let grid = world.get_world();

    for (linenum, line) in grid.iter().enumerate() {
        for block in line.iter() {
            draw_block(&window, block);
        }
        window.mv(linenum as i32, 0);
        println!("");
    }
}

fn main() {
    let mut world = World::new(30);
    world.generate();

    let window = initscr();

    render_world(&window, &world);

    window.refresh();

    window.getch();
    endwin();
}
