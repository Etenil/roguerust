extern crate rand;
extern crate pancurses;

#[macro_use]
extern crate text_io;

mod character;
mod computer;
mod world;

use character::Player;
use computer::Enemy;
use pancurses::{initscr, endwin};
use rand::Rng;
use std::io;

fn main() {
    let window = initscr();
    window.printw("Hello Rust");
    window.refresh();
    window.mv(2, 2);
    window.printw("toto");
    window.refresh();
    window.getch();
    endwin();
}
    // println!(
    //     "=== Welcome to RRL {} the {}! ===\n",
    //     env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION")
    // );

    // let characters: [character::Character; 5] = [
    //     character::Character::new("".to_string(), "Cleric".to_string(), 7, 5, 6, 7),
    //     character::Character::new("".to_string(), "Warrior".to_string(), 10, 5, 5, 5),
    //     character::Character::new("".to_string(), "Hunter".to_string(), 5, 7, 7, 6),
    //     character::Character::new("".to_string(), "Wizard".to_string(), 3, 10, 5, 7),
    //     character::Character::new("".to_string(), "Thief".to_string(), 4, 5, 6, 10),
    // ];

    // let _luck_amount = rand::thread_rng().gen_range(2, 6);

    // println!("You enter the Ephemeral Plane of Creation...");
    // println!("Please enter your name.");

    // let mut input_text = String::new();

    // io::stdin()
    //     .read_line(&mut input_text)
    //     .expect("Failed to read line");
    // let _character_name = input_text.trim();

    // println!("Please select your character type:");
    // for (i, elem) in characters.iter().enumerate() {
    //     print!("\n{}. {}\n\n", i + 1, elem.info());
    // }

    // let mut character_index: usize = 100;
    // while character_index > characters.len() {
    //     character_index = read!();
    // }

    // let mut player = characters[character_index].select(_character_name.to_string(), _luck_amount);

    // play(&mut player);
// }

fn play(player: &mut character::Character) {
    println!("=== Welcome to RRL {} the {}! ===\n", player.name, player.class);
    println!("Your unique stats: {}", player.stats());
}
