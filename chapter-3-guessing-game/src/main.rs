use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    loop {
        println!("Guess the number!");

        let answer = rand::thread_rng().gen_range(1..=100);

        println!("Answer: {}", answer);

        // Needs to be a string to accept UTF8 input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Try to convert the string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, let's try that again!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small! Answer: {}", answer),
            Ordering::Greater => println!("Too big! Answer: {}", answer),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
