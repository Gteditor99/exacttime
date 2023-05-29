use chrono::{DateTime, Local, Timelike, Datelike};
use colored::*;
use std::io::Write;
fn main() {
    loop {
        // Get the current time
        let now: DateTime<Local> = Local::now();
        let time = now.format("%Y:%m:%d:%I (%P):%M:%S:%f").to_string();

        // Clear the previous line
        print!("\r{}", " ".repeat(time.len()));

        // Print the current time with desired styling
        print!("{}", time.blue().bold());

        // Flush the output to ensure immediate display
        std::io::stdout().flush().unwrap();

        // Sleep for a fraction of a second
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
