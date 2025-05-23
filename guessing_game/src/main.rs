use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("Try to guess the number between 1 and 100.");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nPlease input your guess:");

        // Create a new String to store the guess
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string to a number, handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break;
            }
        }
    }
} 