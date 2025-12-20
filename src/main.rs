mod clear_console;
mod constants;
mod members;
mod utils;

use colored::Colorize;
use std::{
    io::{self, Write},
    u8,
};

// use clear_console::clear_terminal_screen;
use crate::utils::user_input;
use constants::ERROR;
use members::staff::staff;
use members::student::student;
use members::teacher::teacher;

fn main() {
    loop {
        // clear_terminal_screen(); //* Clears the console for fresh view */
        println!(
            "{}{}{}",
            "==================".green().bold(),
            "Welcome".cyan().bold(),
            "==================".green().bold()
        );
        println!(
            "{}{}{}",
            "========".green().bold(),
            "College Management Solution".cyan().bold().underline(),
            "========".green().bold()
        );
        println!("Select Option of your Choice:");
        println!("\t1. Manage Student Record (Press 1):");
        println!("\t2. Manage Teachers Record (Press 2):");
        println!("\t3. Manage Staff Record (Press 3):");
        let mut choice: u8 = user_input::input_u8(None, 1, 4);
        match choice {
            1 => student("data/student.csv"),
            2 => teacher("data/teacher.csv"),
            3 => staff("data/staff.csv"),
            _ => println!(
                "{}: Enter a valid choice between 1 to 3!",
                ERROR.red().bold()
            ),
        }
        let mut user_input: String = "".to_string();
        let mut is_valid_input: bool = false;
        /*
         * Prompt the users for exit option,
         * if read and such input found than
         * exit successfully otherwise restart
         */
        while !is_valid_input {
            print!(
                "{}\n\t[Enter {} for {} or {} to exit]: ",
                "Do you want to continue with the main menu?"
                    .purple()
                    .bold(),
                "1".green().bold(),
                "yes".green().bold(),
                "any other".red().bold()
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
        if choice != 1 {
            break;
        }
    }
    println!(
        "{}{}{}",
        "=================".green().bold(),
        "Thank You".cyan().bold(),
        "=================".green().bold()
    );
}
