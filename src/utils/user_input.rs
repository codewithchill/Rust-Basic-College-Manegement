use crate::{constants::ERROR};
use colored::Colorize;
use std::{io::{self, Write}};

pub fn input_u8(statement: Option<&str>, lower_limit: u8, upper_limit: u8) -> u8 {
    let mut user_input: String = "".to_string();
    let mut is_valid_input: bool = false;
    let mut choice: u8 = 0;
    if let Some(text) = statement {
        println!("{}", text);
    }
    while !is_valid_input {
        print!("Enter the number: ");
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
pub fn input_u32(statement: Option<&str>, lower_limit: u32, upper_limit: u32) -> u32 {
    let mut user_input: String = "".to_string();
    let mut is_valid_input: bool = false;
    let mut choice: u32 = 0;
    if let Some(text) = statement {
        println!("{}", text);
    }
    while !is_valid_input {
        print!("Enter the number: ");
        io::stdout().flush().expect("Error in flushing to stdout");
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error in reading input!");
        match user_input.trim().parse::<u32>() {
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
pub fn input_str(statement: Option<&str>, min_len: u16, max_len: u16) -> String {
    let mut user_input: String = String::new();
    if let Some(text) = statement {
        println!("{}", text);
    }
    loop {
        print!("Enter your input: ");
        io::stdout().flush().expect("Error in flushing output");
        user_input.clear();
        if io::stdin().read_line(&mut user_input).is_err() {
            println!("{}: Input Failed!", ERROR.bold().red());
            continue;
        }
        if (user_input.trim().len() > max_len.into()) || (user_input.trim().len() < min_len.into()) {
            println!(
                "{}: Enter a valid string of length between {} to {}! (Current: {})",
                ERROR.red().bold(),
                min_len.to_string().bold().cyan(),
                max_len.to_string().bold().cyan(),
                user_input.trim().len()
            );
        } else {
            break;
        }
    }
    return user_input.trim().to_string();
}
