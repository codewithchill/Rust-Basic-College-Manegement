use super::department::DepartmentType;
use crate::{
    clear_console::clear_terminal_screen,
    constants::{ERROR, MAX_RECORD_PER_SESSION},
    utils::user_input,
};
use colored::Colorize;
use std::io::{self, Write};

pub fn student(filename: &str) {
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
        let mut choice: u8;

        choice = user_input::input_u8(None, 1, 4);
        // while !is_valid_input {
        //     print!("Enter your choice: ");
        //     io::stdout().flush().expect("Failed to flush the output");
        //     user_input.clear();
        //     io::stdin()
        //         .read_line(&mut user_input)
        //         .expect("Failed to get input");
        //     match user_input.trim().parse::<u8>() {
        //         Ok(value) => {
        //             choice = value;
        //             is_valid_input = true;
        //         }
        //         Err(_) => {
        //             println!("{}: Enter a valid number!", ERROR.red().bold());
        //             is_valid_input = false;
        //         }
        //     }
        // }
        // is_valid_input = false;
        match choice {
            1 => record_add(filename),    //* record_add */
            2 => record_view(filename),   //* record_view */
            3 => record_update(filename), //* record_update */
            4 => record_delete(filename), //* record_delete */
            _ => println!(
                "{}: Enter a valid option between 1 to 4!",
                ERROR.red().bold()
            ),
        }
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

struct Student {
    id: u64,
    name: String,
    dept: DepartmentType,
    roll_no: u64,
    year: u8,
    cgpa: f32,
}
// const total_student_fields:u8 = 6; //* will be later used for counting  */
fn record_add(_filename: &str) {
    loop {
        println!(
            "How many student's record do you want to put? [Max = {} per session]",
            MAX_RECORD_PER_SESSION.to_string().bold().underline().cyan()
        );
        let student_record_num: u8 = user_input::input_u8(None, 1, 100);
        // let mut user_input: String = "".to_string();
        // let mut is_valid_input: bool = false;
        // println!("Entered value is: {}", student_record_num); //* Debug
        // let mut student_details: Vec<Student> = Vec::with_capacity(student_record_num as usize);
        // while !is_valid_input {}
        // is_valid_input = false;
        let mut _exit_option: u8 = 0;
        println!(
            "Do you want to exit? [Press {} for {}]",
            "1".green().bold(),
            "yes".green().bold()
        );
        // while !is_valid_input {}
        // exit_option = input_u8(None, 0, upper_limit)
        if _exit_option == 1 {
            break;
        }
    }
    return;
}
fn record_view(filename: &str) {
    println!("File name is: {}", filename.cyan().bold().underline().italic());
}
fn record_delete(filename: &str) {
    println!("File name is: {}", filename.cyan().bold().underline().italic());
}
fn record_update(filename: &str) {
    println!("File name is: {}", filename.cyan().bold().underline().italic());
}
