// Day 1 final project – Guess the Number Game but with some unique twists:

// Limited attempts (like 7 chances).
// Hints after every wrong guess ("Too high!" / "Too low!").
// At the end, show score: number of attempts taken.
// If the player fails, reveal the secret number.
// Add a "Play Again" option.




use rand::Rng; // this comes from the rand crate (a library)
// Rng is a trait that provides methods for generating random numbers
use std::cmp::Ordering; // this brings the ordering enum into scope 
// Ordering can be: less, greater, or equal
// we use it when comparing two values with .cmp()
use std::io;

// external crates 

// a "crate" in rust = a library or package of code
// rust has a standard library (like std::io, std::cmp), which comes built-in
// but you can also use external crates made by others, published on crates.io
// eg:
// rand (for random numbers)
// serde (for serialization, like JSON handling)
// tokio (for async programming)

// enums

// "enum" = one type, many possible forms (like multiple choices)
// useful when you want a variable to represent different possible states
// eg in real life
// traffic light = red, yellow, green
// payment mode = cash, card, UPI


//External crate = library you bring in from outside (like rand).
//Enum = a custom type with multiple possible forms (good for state handling).

fn main() {
    println!("welcome to the guess the number game!");

    loop {
        // generate random number between 1 and 100
        let secret_number = rand::thread_rng().gen_range(1..=100); 
        // here thread_rng() gives us a random number generator
        // gen_range(1..=100) picks a number between 1 and 100

        // traits = shared behavior contracts
        // types (structs, enums) must implement them
        // they can have default methods
        // they allow polymorphism in rust (different types, same behavior)

        let mut attempts = 0;
        let max_attempts = 7;

        println!("i have chosen a number between 1 and 100.");
        println!("u have {} attempts to guess it. good luck!", max_attempts);

        while attempts < max_attempts {
            println!("\npls enter ur guess:");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("failed to read input");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("pls enter a valid number!");
                    continue;
                }
            };

            attempts += 1;

            println!("u guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too small!"),
                Ordering::Greater => println!("too big!"),
                Ordering::Equal => {
                    println!("congratulations! u guessed it in {} attempts!", attempts);
                    break;
                }
            }

            if attempts == max_attempts {
                println!("\nu have used all {} attempts!", max_attempts);
                println!("the secret number was: {}", secret_number);
            }
        }

        
        println!("\ndo u want to play again? (yes/no)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("failed to read input");

        if play_again.trim().to_lowercase() != "yes" {
            println!("thanks for playing! goodbye!");
            break;
        }
    }
}