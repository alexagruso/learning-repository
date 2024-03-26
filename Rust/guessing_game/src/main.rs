use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("I'm thinking of a number...'");
    println!("Take a guess!");

    let mut guesses = 0;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        guesses += 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low..."),
            Ordering::Greater => println!("Too high..."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }

    println!("It took you {guesses} guesses");
}
