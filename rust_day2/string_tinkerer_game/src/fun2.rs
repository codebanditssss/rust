// Passing References to Functions  

//Problem: Ownership Moves When Passing Arguments

// When you pass a non-Copy type (like String) into a function, ownership moves:

fn consume_string(s: String) {
    println!("Inside function: {}", s);
}

fn main() {
    let name = String::from("Rustacean");
    consume_string(name);  
    println!("{}", name);   // Error: value borrowed after move
}


// Error → "borrow of moved value".
// Reason: String is not Copy. Ownership moved into consume_string().
