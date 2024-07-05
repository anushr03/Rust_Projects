use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please enter your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You've guessed: {}",guess);
}
