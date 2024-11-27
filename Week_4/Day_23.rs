use std::thread;
use std::time::Duration;
use std::io;
use colored::*;

fn countdown(minutes: u64, seconds: u64) {
    let total_seconds = minutes * 60 + seconds;  // convert minutes to seconds
    
    for remaining in (1..=total_seconds).rev() {
        // Color logic: 
        // Green when plenty of time left (>50%)
        // Yellow when getting close (25-50%)
        // Red when almost done (<25%)
        let colored_time = match remaining as f64 / total_seconds as f64 {
            x if x > 0.5 => format!("{}", remaining).green(),
            x if x > 0.25 => format!("{}", remaining).yellow(),
            _ => format!("{}", remaining).red()
        };
        
        println!("{} seconds remaining", colored_time);
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("{}", "Time's up!".bright_green().bold());
}

fn main() {
    println!("{}", "Welcome to the Countdown Timer!".bright_blue());
    
    let minutes = get_user_input("Enter the number of minutes to countdown:");
    let seconds = get_user_input("Enter the number of seconds to countdown:");
    
    countdown(minutes, seconds);
}

fn get_user_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt.bright_cyan());
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("{}", "Invalid input. Please enter a valid number.".red());
                continue;
            }
        }
    }
} 
 
