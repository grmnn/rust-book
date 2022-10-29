use rand::{thread_rng, Rng};
use std::io;

use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    let mut rng = thread_rng();

    let random_number = rng.gen_range(1..10);
    println!("Our random number is: {random_number}");
    loop {
        println!("Guess the random number!");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read from stdin");
            
        let user_guess = user_guess.trim();

        match user_guess.parse::<u32>() {
            Ok(i) => match i.cmp(&random_number) {
                Equal => {
                    println!("You've guessed correctly! You won!");
                    break;
                }
                Greater => println!("Your guess was too high :( Try again!"),
                Less => println!("Your guess was too low :( Try again!"),
            },
            Err(..) => println!("User input was not a number: {user_guess}"),
        }
    }
}
