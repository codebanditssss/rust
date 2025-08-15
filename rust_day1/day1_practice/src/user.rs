// use std::io; // input/output module
// fn main() {
//     let mut name = String::new(); // create empty string
//     println!("Enter your name:");

//     io::stdin()
//     .read_line(&mut name) // read input into variable
//     .expect("Failed to read line"); // handle error if any

//     println!("Hello, {}!", name); 
// }

// String::new() -> creates empty string
// read_line(&mut name) -> fills variable with user input
// expect(...) -> crashes with message if reading fails
// mut is required because the variable will be updated

// o/p: hello, kiki
// !
// when we type kiki and press enter, the input actually contains the newline character \n at the end
// so name is actually "kiki\n" internally
// when we print it with println!, rust keeps the \n
// use .trim() to remove the newline and any extra whitespace
// .trim() removes leading and trailing whitespace, including \n and spaces
// println!("Hello, {}!", name.trim());

// use std::io;
// fn main() {
//     let mut age = String::new();
//     println!("Enter your age:");

//     io::stdin()
//         .read_line(&mut age)
//         .expect("Failed to read line");

//     let age: i32 = age.trim().parse().unwrap(); 
//     println!("Your age is {}", age);
// }

use std::io;

fn main() {
    let mut name = String::new();
    println!("enter ur name:");
    io::stdin().read_line(&mut name).expect("failed to read line");

    let mut age = String::new();
    println!("enter ur age:");
    io::stdin().read_line(&mut age).expect("failed to read line");

    let age: i32 = age.trim().parse().unwrap(); 

    println!("hello, {}! u are {} years old.", name.trim(), age);
}


// common mistakes:

// Forget mut → cannot pass variable to read_line
// Forget .trim() → parse may fail
// Forget .parse() → type mismatch
// Forget .unwrap() → you have a Result instead of integer
// Shadowing vs mutation confusion:
// let age = age.trim().parse().unwrap(); //  shadowing
// Inputting wrong type → program panics
// Using &str instead of String → cannot store mutable input