use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("The correct number is: {}", correct);
    println!("Hey, guess a number 1-10:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = guess.trim().parse().expect("Error parsing guess.");

        // if expression
        // let mut message = if correct < guess {
        //     String::from("You guessed too high.")
        // } else if correct > guess {
        //     String::from("You guessed too low.")
        // } else {
        //     String::from("You guessed the correct number.")
        // };

        // match expression
        let message = match guess.cmp(&correct) {
            Ordering::Greater => "You guessed too high.",
            Ordering::Less => "You guessed too low.",
            Ordering::Equal => {
                println!("You guessed the correct number.");
                break;
            }
        };

        println!("{message}");
    }
}
