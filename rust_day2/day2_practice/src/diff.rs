// Tuple Structs vs Named Structs

// In Rust, both are ways to group multiple values together, but they differ in how you access and use them.


// 1. Named Structs

// Fields are given names.
// Best for readability and when meaning matters.
// Access fields using dot notation.


struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Width: {}, Height: {}", rect.width, rect.height);
}


// Easy to read: you know width and height by name.