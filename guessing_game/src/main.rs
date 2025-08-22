use std::io;

fn main() {
    println!("Guess a number!");

    println!("Input your guess:");

    // mut means that its a mutable variable, by default in rust
    // variables are immutable (const)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("you guessed: {}", guess)
}
