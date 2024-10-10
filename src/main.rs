use std::io;

fn main() {
    println!("Hey, guess a number:");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading input");

    println!("You guessed: {}", num.trim());
}
