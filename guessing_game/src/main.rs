use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number!");
    let num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", num);

    loop {
        println!("Input your guess:");

        // mut means that its a mutable variable, by default in rust
        // variables are immutable (const)
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // expect is like the failure handling thing
        // this is called shadowing (reuse the guess variable and change it to another type)
        let guess: u32 = match guess.trim().parse() {
            // match statement is similar to an if statement ?
            Ok(num) => num, // if parse can turn the string to a number
            // the _ is a catch-all values, in this context means we want to catch all error types
            Err(_) => continue, // otherwise
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&num) {
            // each of these options is called an arm (like a switch statement ?)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
