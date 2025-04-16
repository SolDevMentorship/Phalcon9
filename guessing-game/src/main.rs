use rand::prelude::*;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");
    println!("Welcome to Guessing Game!");

    // Step 1: Computer guess a random number and
    // Step 2: stores in memory
    let mut rng = rand::rng();
    let computer_rand_number: i32 = rng.random_range(1..=100);
    println!("{computer_rand_number}");

    println!("Guess a Number:");

    loop {
        // Step 3: ask the user to guess the number stored in the memory
        let mut guess = String::new();
        // Step 4: Users inputs number
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        // Step 5: Computer compares users input wih computer's initial guess
        // Step 5a: Convert user input from string (Terminal parse value as string) to Interger.
        let parsed_guess: i32 = guess.trim().parse().expect("Please input a real number!");

        // Step 6: If number is correct inform user wins, else inform user if guess lower or higher
        let cmp_guess = parsed_guess.cmp(&computer_rand_number);
        match cmp_guess {
            Ordering::Less => println!("Your Guess is less! Guess Again"),
            Ordering::Greater => println!("Your Guess is greater! Guess Again"),
            Ordering::Equal =>{ 
                println!("Your Guess is correct.");
                break;
            },
        }

        // Step 7: Start from step 1
    }
}
