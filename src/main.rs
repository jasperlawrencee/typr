use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    println!("=== RUST TYPING GAME ===");
    println!("Type each word that appears and press Enter.");
    println!("Press Ctrl+C at any time to quit.");
    println!("Press Enter to start...");
    
    let _ = io::stdin().read_line(&mut String::new()).unwrap();
    
    let words = vec![
        "rust", "programming", "language", "compiler", "memory", 
        "safety", "performance", "concurrency", "traits", "ownership",
        "borrowing", "lifetime", "crate", "module", "function",
        "structure", "enumeration", "pattern", "matching", "iterator",
        "closure", "reference", "mutable", "immutable", "stack",
        "heap", "allocation", "variable", "constant", "type",
    ];
    
    let mut rng = rand::thread_rng();
    let mut correct_count = 0;
    let mut total_count = 0;
    let mut total_time = Duration::new(0, 0);
    
    // provides 10 random words
    const MAX_WORDS: i8 = 10;
    for _ in 0..MAX_WORDS {
        let word = words.choose(&mut rng).unwrap();
        print!("{} > ", word);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        let start = Instant::now();
        io::stdin().read_line(&mut input).unwrap();
        let elapsed = start.elapsed();
        
        let input = input.trim();
        let correct = input == *word;
        
        if correct {
            println!("✓ Correct! ({:.2} seconds)", elapsed.as_secs_f32());
            correct_count += 1;
        } else {
            println!("✗ Wrong! Expected: '{}', got: '{}' ({:.2} seconds)", 
                     word, input, elapsed.as_secs_f32());
        }
        
        total_count += 1;
        total_time += elapsed;
    }
    
    let accuracy = (correct_count as f32 / total_count as f32) * 100.0;
    let avg_time = total_time.as_secs_f32() / total_count as f32;
    let wpm = (correct_count as f32 / (total_time.as_secs_f32() / 60.0)).round();
    
    println!("\n=== RESULTS ===");
    println!("Words: {}/{}", correct_count, total_count);
    println!("Accuracy: {:.1}%", accuracy);
    println!("Average time per word: {:.2} seconds", avg_time);
    println!("Words per minute: {}", wpm);
    
    if accuracy >= 90.0 {
        println!("\nExcellent typing skills!");
    } else if accuracy >= 70.0 {
        println!("\nGood job! Keep practicing.");
    } else {
        println!("\nKeep practicing to improve your speed and accuracy.");
    }
}