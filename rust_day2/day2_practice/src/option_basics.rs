// Option<T> for Safe Null Handling
// No null pointers in Rust! Use Option<T> instead

// The problem with null in other languages:
// - Accessing null causes crashes
// - Easy to forget null checks
// - "billion-dollar mistake" according to its inventor

// Rust's solution: Option<T> enum
// enum Option<T> {
//     Some(T),  // has a value of type T
//     None,     // no value
// }

fn main() {
    // Creating Option values
    let some_number = Some(5); // Option<i32> with value 5
    let some_string = Some("hello"); // Option<&str> with value "hello"
    let no_value: Option<i32> = None; // Option<i32> with no value
    
    println!("Created some Option values!");
    
    // You MUST handle both cases with match
    match some_number {
        Some(value) => println!("Got number: {}", value), // extract the value
        None => println!("No number found"),
    }
    
    match no_value {
        Some(value) => println!("Got number: {}", value),
        None => println!("No number found"), // this will run
    }
    
    // Practical example: safe division
    let result1 = safe_divide(10, 2); // returns Some(5)
    let result2 = safe_divide(10, 0); // returns None (can't divide by zero)
    
    handle_division_result(result1);
    handle_division_result(result2);
    
    // Example: finding item in array
    let numbers = vec![1, 2, 3, 4, 5];
    let found = find_number(&numbers, 3); // returns Some(2) - index where 3 is found
    let not_found = find_number(&numbers, 10); // returns None - 10 not in array
    
    match found {
        Some(index) => println!("Found number at index: {}", index),
        None => println!("Number not found"),
    }
    
    match not_found {
        Some(index) => println!("Found number at index: {}", index),
        None => println!("Number not found"), // this runs
    }
    
    // Example: parsing user input safely
    let valid_input = "42";
    let invalid_input = "not_a_number";
    
    let parsed1 = parse_number(valid_input); // returns Some(42)
    let parsed2 = parse_number(invalid_input); // returns None
    
    handle_parsed_number(parsed1);
    handle_parsed_number(parsed2);
    
    // Common Option methods (alternatives to match)
    let maybe_name = Some("Alice");
    
    // unwrap() - gets the value but panics if None (dangerous!)
    // let name = maybe_name.unwrap(); // only use if you're 100% sure it's Some
    
    // unwrap_or() - gets the value or provides a default
    let name = maybe_name.unwrap_or("Unknown"); // gets "Alice"
    println!("Name: {}", name);
    
    let empty: Option<&str> = None;
    let default_name = empty.unwrap_or("Guest"); // gets "Guest" since it's None
    println!("Default name: {}", default_name);
    
    // expect() - like unwrap() but with custom error message
    let important_value = Some(100);
    let value = important_value.expect("This should never be None!"); // gets 100
    println!("Important value: {}", value);
}

// Safe division that returns Option instead of crashing
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None // can't divide by zero
    } else {
        Some(a / b) // return the result wrapped in Some
    }
}

fn handle_division_result(result: Option<i32>) {
    match result {
        Some(value) => println!("Division result: {}", value),
        None => println!("Cannot divide by zero!"),
    }
}

// Find a number in a vector, return its index
fn find_number(numbers: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() { // iterate with index
        if number == target {
            return Some(index); // found it! return the index
        }
    }
    None // not found
}

// Parse a string to a number safely
fn parse_number(input: &str) -> Option<i32> {
    match input.parse::<i32>() { // parse() returns Result, we convert to Option
        Ok(number) => Some(number), // parsing succeeded
        Err(_) => None, // parsing failed
    }
}

fn handle_parsed_number(result: Option<i32>) {
    match result {
        Some(number) => println!("Parsed number: {}", number),
        None => println!("Failed to parse number"),
    }
}

/*
What we built here:

PROBLEM: In languages like C/Java, null values crash your program when you try to use them.
You write `user.name` but user is null = CRASH! This happens all the time.

RUST SOLUTION: Option<T> forces you to check if something exists before using it.

Basic idea:
- Some(value) = "I have something"  
- None = "I have nothing"

What our functions do:

1. safe_divide(10, 2) 
   - Normal division would crash on divide-by-zero
   - Our version returns None for zero, Some(result) for valid division
   - No crashes possible!

2. find_number(&vec, 3)
   - Searches for number 3 in the vector
   - Returns Some(index) if found, None if missing  
   - No "index out of bounds" errors

3. parse_number("42")
   - Tries to convert string to number
   - Returns Some(42) if valid, None if invalid
   - No parsing crashes

Why this is genius:
- Compiler FORCES you to handle both Some and None cases
- Impossible to accidentally use a None value
- No surprise crashes from missing data

Safe methods we used:
- unwrap_or("default") = get value or use backup
- expect("message") = get value or crash with custom error  
- match = handle both Some and None explicitly

The key insight: Instead of hoping data exists and crashing when it doesn't,
Option makes "missing data" part of the type system. You can't ignore it.

Real world example:
Instead of: user.email (might be null, might crash)
Use: match user.email { Some(addr) => send_mail(addr), None => show_error() }

Common mistakes:
❌ some_option.unwrap()           // crashes if None, avoid this
❌ if option == None              // wrong syntax  
❌ not handling None case         // compiler won't let you

✅ match option { Some(x) => use_x(), None => handle_missing() }
✅ option.unwrap_or(fallback)     // safe with default
✅ always handle both cases       // compiler enforces this
*/
