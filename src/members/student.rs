use colored::Colorize;

pub fn student(filename: &str) {
    loop {
        println!(
            "{}{}{}",
            "========".green().bold(),
            "Student Management Solution".cyan().bold().underline(),
            "========".green().bold()
        );
        println!("Please choose from the options below:");
		println!("\t1. Add a Student's Record (Press 1):");
		println!("\t2. View Student Record (Press 2):");
		println!("\t3. Update a student record (Press 3):");
		println!("\t4. Delete a Student Record (Press 4):");
        let mut is_valid_input:bool = false;
        while !is_valid_input {
        }
        is_valid_input = false;
        print!("Do you want to continue with student portal or go back?\n\t(Enter 1 to continue or press any key to go back!): ");
    }
}
