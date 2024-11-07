#![warn(missing_docs)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// # Guessing game

/// Get input from the user
///
/// Returns:
/// - `i32`: The user's guess
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim whitespace and convert the string to an integer
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// Runs the guessing game
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_input();
        print!("You guessed: {}. ", guess);

        // Compare the guess with the secret number
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
