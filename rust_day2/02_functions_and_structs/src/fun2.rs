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

// fn add_exclamation(s: &mut String) {
//     s.push_str("!");
// }

// fn main() {
//     let mut greeting = String::from("Hello");
//     add_exclamation(&mut greeting);
//     println!("{}", greeting);  // Hello!
// }

// Only one mutable reference at a time allowed (to prevent data races).
// You need both the variable and the reference to be mutable (let mut and &mut).



// Example with Simple Type (i32)

// For Copy types (like integers, floats, bool), ownership isn’t moved — but borrowing still works:
fn add_one(n: &mut i32) {
    *n += 1;  // dereference to change value
}

fn main() {
    let mut x = 10;
    add_one(&mut x);
    println!("{}", x);   //  11
}


//Common Mistakes Here

//Forgetting & when passing → ownership move error.
//Trying multiple mutable borrows:
// let mut s = String::from("hi");
// let r1 = &mut s;
// let r2 = &mut s;  // error: cannot borrow `s` as mutable more than once

//Mixing immutable and mutable borrows at the same time:

// let mut s = String::from("hi");
// let r1 = &s;
// let r2 = &mut s;   //  error

