use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");
    println!("Press q to quit any time.");
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop{
        println!("Please enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //added functionality to quit game anytime. trim() because &str add newline character     
        if guess.trim() == "q"{
            println!("Goodbye");
            break;
        }

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You've guessed: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too Small!"),
            Ordering::Greater=> println!("Too Big"),
            Ordering::Equal=> {
                println!("You Win");
                break;
            }
        }
    }
}
