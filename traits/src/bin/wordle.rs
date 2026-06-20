use colored::Colorize;

use std::io::{self, Write};

fn main() {
    let world = "trait";
    let input = io::stdin();

    for _ in 1..=6 {
        let mut user_input = String::new();

        input.read_line(&mut user_input).expect("Error");

        for (word_character, user_character) in world.chars().zip(user_input.trim().chars().take(5))
        {
            if word_character == user_character {
                print!("{}|", format!("{user_character}").on_green())
            } else if world.contains(user_character) {
                print!("{}|", format!("{user_character}").on_yellow())
            } else {
                print!("{}|", format!("{user_character}").on_black())
            }

            io::stdout().flush().unwrap();
        }

        println!();

        if world == user_input.trim() {
            print!("you got it! The word is {world}");
        }
    }
}
