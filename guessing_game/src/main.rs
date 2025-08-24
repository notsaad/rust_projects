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
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("you guessed: {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
