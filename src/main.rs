use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;

const TEST_PASSAGES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "Never underestimate the power of a good book and a warm cup of coffee.",
    "Programming in Rust can be challenging yet incredibly rewarding for many.",
    "Learning by doing is an effective way to master new skills and concepts.",
    "A journey of a thousand miles begins with a single step towards the horizon.",
];

fn main() {
    println!("Generating a random passage...");
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..TEST_PASSAGES.len());
    let target_text = TEST_PASSAGES[random_index];
    
    println!("\nType the following text:");
    println!("{}", target_text);
    println!("\nPress Enter when ready to start typing...");
    
    // Wait for user to press Enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    println!("\nStart typing now!");
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    // Start timing
    let start_time = Instant::now();
    
    // Read user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    
    // Stop timing
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    
    // Calculate results
    let user_text = user_input.trim();
    let user_words: Vec<&str> = user_text.split_whitespace().collect();
    
    let time_minutes = duration.as_secs_f64() / 60.0;
    let wpm = if time_minutes > 0.0 {
        (user_words.len() as f64 / time_minutes) as u32
    } else {
        0
    };
    
    // Calculate accuracy
    let mut correct_chars = 0;
    let min_len = target_text.len().min(user_text.len());
    for i in 0..min_len {
        if target_text.chars().nth(i) == user_text.chars().nth(i) {
            correct_chars += 1;
        }
    }
    let accuracy = if target_text.len() > 0 {
        (correct_chars as f64 / target_text.len() as f64) * 100.0
    } else {
        0.0
    };
    
    // Display results
    println!("\n--- Results ---");
    println!("Time: {:.2} seconds", duration.as_secs_f64());
    println!("WPM: {}", wpm);
    println!("Accuracy: {:.1}%", accuracy);
    println!("Characters: {}/{} correct", correct_chars, target_text.len());
}
