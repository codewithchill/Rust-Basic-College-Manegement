use crate::{clear_console::clear_terminal_screen, constants::ERROR};
use colored::Colorize;
use std::io::{self, Write};

pub fn student(_filename: &str) {
    loop {
        clear_terminal_screen();
        println!(
            "{}{}{}",
            "===============".green().bold(),
            "Student Management Solution".cyan().bold().underline(),
            "===============".green().bold()
        );
        println!("Please choose from the options below:");
        println!("\t1. Add a Student's Record (Press 1):");
        println!("\t2. View Student Record (Press 2):");
        println!("\t3. Update a student record (Press 3):");
        println!("\t4. Delete a Student Record (Press 4):");
        let mut is_valid_input: bool = false;
        let mut user_input: String = "".to_string();
        let mut choice: u8 = 0;
        // while !is_valid_input {

        // }
        // is_valid_input = false;

        while !is_valid_input {
            print!(
                "{}\n\t(Enter {} to {} or press any key to go back!): ",
                "Do you want to continue with student portal or go back?"
                    .purple()
                    .bold(),
                "1".green().bold(),
                "continue".bold().green()
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
            println!(
                "{}{}{}",
                "===============:".green().bold(),
                "Thank You".cyan().bold().underline(),
                ":===============".green().bold()
            );
            return;
        }
    }
}
