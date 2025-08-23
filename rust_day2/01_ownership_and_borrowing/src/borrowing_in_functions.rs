fn calculate_length(s: &String) -> usize {
    s.len() // can read, but not modify
}


// usize

// The return type. usize is an unsigned integer type sized 
// to hold indices and lengths on the current platform (on 64-bit machines it’s 64 bits, on 32-bit it’s 32 bits).
// Here it means the function returns the length (number of bytes/chars as usize).


// &String


// The type of the parameter s. This is a reference to a String.
// & means “borrow” (pass a reference, not ownership).
// String is the heap-allocated growable string type in Rust.
// So s: &String means: the function receives a borrowed reference to a String (read-only here).





fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}.", s1, len);
}

// passing references instead of ownership is extremely common
// works because &s1 is borrowed. ownership stays with main


// what is &String?

// & means a reference
// a reference lets us borrow a value without taking ownership
// so s: &String means: "this function will use a reference to a string, but it wont own it


// ownership stays in main

// in main, s1 owns "hello"
// when we call calculate_length(&s1), we are borrowing s1 (not moving it)
// that means after the function call, s1 is still valid in main


// why does this work?


// if we passed s1 without &, ownership would move to the function
// and we wouldn’t be able to use s1 in println! anymore
// by passing &s1, the function gets read-only access. ownership stays in main


// what can you do with &String?

// u can read the value (s.len(), s.chars(), etc).
// u cannot modify it, bcuz a normal reference is immutable by default
// if u tried to write s.push_str("world"), it would fail

//-----------------------------------------------------------------------------------

// Visualizing it

// Think of s1 as a book in main.
// If we moved ownership, we gave away the book to the function. After that, we couldn’t read it anymore.
// If we borrowed it with &s1, it’s like we let the function read the book, but it must return it after reading. So main still has the book.


// In short:

// & = borrow, don’t own.
// Function can read it, but can’t change it.
// The original variable (s1) is still usable after the function call.



//-------------------------------------------------------------------------------------