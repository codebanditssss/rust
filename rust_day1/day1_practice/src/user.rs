use std::io; // input/output module
fn main() {
    let mut name = String::new(); // create empty string
    println!("Enter your name:");
    
    io::stdin()
    .read_line(&mut name) // read input into variable
    .expect("Failed to read line"); // handle error if any

    println!("Hello, {}!", name); 
}

// String::new() -> creates empty string
// read_line(&mut name) -> fills variable with user input
// expect(...) -> crashes with message if reading fails
// mut is required because the variable will be updated