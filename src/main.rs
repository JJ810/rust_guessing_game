use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    // println!("The correct number is: {}", correct);
    println!("Hey, guess a number 1-10:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };

        // if expression
        // let mut message = if correct < guess {
        //     String::from("You guessed too high.")
        // } else if correct > guess {
        //     String::from("You guessed too low.")
        // } else {
        //     String::from("You guessed the correct number.")
        // };

        // match expression
        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low."),
            Ordering::Equal => {
                println!("You guessed the correct number.");
                break;
            }
        };
    }
}
