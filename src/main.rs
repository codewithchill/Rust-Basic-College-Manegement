use colored::Colorize;
use std::{
    io::{self, Write},
    u8,
};

const ERROR: &str = "ERROR";
// const SUCCESS: &str = "SUCCESS";

fn main() {
    let mut choice: u8 = 0;
    let mut user_input: String = "".to_string();
    let mut is_valid_input: bool = false;
    loop {
        println!(
            "{}{}{}",
            "==================".green().bold(),
            "Welcome!".cyan().bold(),
            "==================".green().bold()
        );
        while !is_valid_input {
            print!(
                "{}[Press {} for yes or any other to exit]: ",
                "Do you want to continue with the main menu?",
                "1".green().bold()
            );
            io::stdout().flush().expect("Failed to flush the output");

            user_input.clear();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to input the choice");

            match user_input.trim().parse::<u8>() {
                Ok(value) => {
                    choice = value;
                    is_valid_input = true;
                }
                Err(_) => {
                    println!("{}: Enter a valid choice!", ERROR.red().bold());
                    is_valid_input = false;
                }
            }
        }
        is_valid_input = false;
        println!(
            "{}{}{}",
            "=================".green().bold(),
            "Thank You!".cyan().bold(),
            "=================".green().bold()
        );
        if choice != 1 {
            break;
        }
    }
}
