use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        // creating a var and setting it's value to an empty string
        let mut guess = String::new();

        // this reads the input from the player
        io::stdio()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // redeclaring the guess var here is a shadow copy
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compares the guess to the secret number
        match guess.cmp(&secret_number) {
            // Ordering enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }
}