use std::process::Command;

//* I totally dont understand this code but using for my project UI */
pub fn clear_terminal_screen() {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status()
    } else {
        // For Unix-like systems (Linux, macOS)
        Command::new("clear").status()
    };

    // Fallback to ANSI escape sequence if the above commands fail
    if result.is_err() {
        print!("\x1B[2J\x1B[1;1H");
    }
}   