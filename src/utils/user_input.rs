use crate::constants::ERROR;
use colored::Colorize;
use std::io::{self, Write};

pub fn input_u8(statement: Option<&str>, lower_limit: u8, upper_limit: u8) -> u8 {
    let mut user_input: String = "".to_string();
    let mut is_valid_input: bool = false;
    let mut choice: u8 = 0;
    if let Some(text) = statement {
        println!("{}", text);
    }
    while !is_valid_input {
        print!("Enter the number of your choice: ");
        io::stdout().flush().expect("Error in flushing to stdout");
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error in reading input!");
        match user_input.trim().parse::<u8>() {
            Ok(value) => {
                is_valid_input = true;
                choice = value;
            }
            Err(_) => {
                is_valid_input = false;
                println!(
                    "{}: Enter a valid number input between {} to {}!",
                    ERROR.red().bold(),
                    lower_limit.to_string().cyan().bold(),
                    upper_limit.to_string().cyan().bold()
                );
                continue;
            }
        }
        if (choice > upper_limit) || (choice < lower_limit) {
            println!(
                "{}: Enter a valid number between {} to {}!",
                ERROR.red().bold(),
                lower_limit.to_string().bold().cyan(),
                upper_limit.to_string().bold().cyan()
            );
            is_valid_input = false;
        }
    }
    return choice;
}
