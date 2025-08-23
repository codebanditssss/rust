// Tuple Structs vs Named Structs

// In Rust, both are ways to group multiple values together, but they differ in how you access and use them.


// 1. Named Structs

// Fields are given names.
// Best for readability and when meaning matters.
// Access fields using dot notation.


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect = Rectangle { width: 30, height: 50 };
//     println!("Width: {}, Height: {}", rect.width, rect.height);
// }


// Easy to read: you know width and height by name.


// 2. Tuple Structs

// Fields don’t have names, only positions.
// Useful for lightweight grouping where naming isn’t necessary.
// Access fields using .0, .1, .2...

struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);
    println!("Red: {}, Green: {}, Blue: {}", black.0, black.1, black.2);
}


// Here Color(0,0,0) is like a "RGB color" with positional values.


// Key Differences

// Named struct → more semantic clarity.
// Tuple struct → more compact, less descriptive.
// Tuple structs are often used for wrapping types (e.g., struct Meters(u32); to differentiate from struct Seconds(u32);).


// Common Mistakes

// Mixing tuple vs named structs unintentionally.
// Forgetting .0, .1 access in tuple struct.
// Shadowing struct field names while using named structs.