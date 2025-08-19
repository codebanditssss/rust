// Passing References to Functions  

//Problem: Ownership Moves When Passing Arguments

// When you pass a non-Copy type (like String) into a function, ownership moves:

// fn consume_string(s: String) {
//     println!("Inside function: {}", s);
// }

// fn main() {
//     let name = String::from("Rustacean");
//     consume_string(name);  
//     println!("{}", name);   // Error: value borrowed after move
// }


// Error → "borrow of moved value".
// Reason: String is not Copy. Ownership moved into consume_string().


// Solution: Borrowing (&)

// Instead of transferring ownership, you can borrow the value:

// fn borrow_string(s: &String) {
//     println!("Inside function: {}", s);
// }

// fn main() {
//     let name = String::from("Rustacean");
//     borrow_string(&name);  
//     println!("{}", name);   // Works fine, still usable
// }

// Explanation:
// &String → means “reference to String”.
// Function doesn’t take ownership, just borrows it.
// Caller keeps ownership after the function call.





//Mutable References (&mut)

//Sometimes you want to modify the value inside a function. For that, you need a mutable borrow.

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut greeting = String::from("Hello");
    add_exclamation(&mut greeting);
    println!("{}", greeting);  // Hello!
}

// Only one mutable reference at a time allowed (to prevent data races).
// You need both the variable and the reference to be mutable (let mut and &mut).