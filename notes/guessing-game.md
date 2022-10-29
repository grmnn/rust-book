# Guessing Game

In this project, we want to create a simple guessing game, where we set a random number between 1 and 100, prompt the player to enter a guess and after the guess, the program gives an answer wether the guessed number was to high, to low or it was correct. 

First we will try to figure this out on our with the help of Rust documentation. 

To make this project work we need following steps:

- Generate a random number
- Print a indication that the user can now enter a number
- Figure out how we can read user input in the terminal and save this input into a variable
- check if the entered input is to high, to low or is the same as our generated number
- In case the input is wrong, let the user guess again, so we need a loop.
- In case the input is correct, let the user guessed correctly and stop the program
  - Maybe ask the user if he wants to play again?


## Generate a random number
To generate a random number we need to use the crate `rand` which can be added to our `Cargo.toml` file in the `[dependencies]` section.

```toml
[dependencies]
rand = "0.8.5"
```

Now we have various [different functions](https://docs.rs/rand/latest/rand/trait.Rng.html#) available.

In our case we can generate a random number like so: 

```rust
use rand::{thread_rng, Rng};
fn main() {    
    let mut rng = thread_rng();
    let random_number = rng.gen_range(1..100);
    println!("The random number is: {random_number}");
}
```

When running this via `cargo run` we can see, every time we run it, we get a new message printed to the terminal with a different random number between 1 and 100. 

## Get the user input

To get a user input we use the `std::io` module which is used to *read* and *write* input and output.

We create an empty mutable variable which will hold the guessed number.
```rust
let mut user_guess = String::new();
```

Next we will read the users input and save it to the variable we created before. *Warning:* expect() Returns the contained Ok value, because this function may panic, its use is generally discouraged but it is fine for now.
```rust
io::stdin().read_line(&mut user_guess).expect("Failed to read from stdin");
```

Now that we have the entered number, we call `.trim()`, to make sure to have the bare value without any additional spaces.
```rust
let user_guess = user_guess.trim();
```

## Compare user input with generated number
Now that we have the user input, we can start comparing it the generated random number.

To do this, we first need to make sure that we have the same type as the generated number. Therefore we call `user_guess.parse` with a `match` expression. `match` let us run code conditionally. This means, when we parse the entered guess and it's not a number, we print the message `println!("User input was not a number: {user_guess}")`. 

If the parsing was successful and we have a number, we make a comparison with a second `match` expression. Here we use the `std::cmp` module which contains various tools to order and compare values. We use the `Ordering` functionality so we are able to check if the comparison outcome is `Equal`, `Greater` or `Less` and print out the appropriate message.

```rust
match user_guess.parse::<u32>() {
    Ok(i) => {
        match i.cmp(&random_number) {
            Equal => println!("You've guessed correctly! You won!"),
            Greater => println!("Your guess was too high :( Try again!"),
            Less => println!("Your guess was too low :( Try again!"),
        }
    },
    Err(..) => println!("User input was not a number: {user_guess}"),
}
```
## Multiple guesses
Because in this case we are only able to guess once before the program ends, we will need to make sure that the user can guess again until he guessed the correct number. This is why we will use `loop`

```rust
loop {
    println!("Guess the random number!");
    let mut user_guess = String::new();
    io::stdin().read_line(&mut user_guess).expect("Failed to read from stdin");
    let user_guess = user_guess.trim();
    
    match user_guess.parse::<u32>() {
        Ok(i) => {
            match i.cmp(&random_number) {
                Equal => {
                    println!("You've guessed correctly! You won!");
                    break;
                },
                Greater => println!("Your guess was too high :( Try again!"),
                Less => println!("Your guess was too low :( Try again!"),
            }
        },
        Err(..) => println!("User input was not a number: {user_guess}"),
    }
}
```

As you can see we wrapped our program, after generating our random number, in a `loop` which runs our program over and over again until we get into the `Equal` condition where we call `break` to stop the loop and our program. 

## Full program

(I have reduced the random number to a number between 1 and 10 and print out the number to guess so we don't have to guess 10 times at max to see if our `Equals` case works.)

```rust
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
```
