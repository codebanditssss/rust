// the ? operator for error propagation
// makes error handling clean and readable

use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== the ? operator for error propagation ===\n");
    
    // the ? operator automatically propagates errors up the call stack
    // it's syntactic sugar for early return on error
    
    // example 1: basic ? usage
    println!("1. basic ? operator usage:");
    match process_user_data() {
        Ok(result) => println!("success: {}", result),
        Err(e) => println!("error: {}", e),
    }
    
    // example 2: chaining operations with ?
    println!("\n2. chaining operations:");
    match calculate_from_files() {
        Ok(result) => println!("calculation result: {}", result),
        Err(e) => println!("calculation failed: {}", e),
    }
    
    // example 3: using ? in main function
    println!("\n3. using ? in main:");
    let config = load_config()?; // this can propagate error out of main
    println!("loaded config: {}", config);
    
    // example 4: converting between error types
    println!("\n4. automatic error conversion:");
    match mixed_error_function() {
        Ok(msg) => println!("mixed operations success: {}", msg),
        Err(e) => println!("mixed operations failed: {}", e),
    }
    
    // show the difference between ? and manual error handling
    comparison_example();
    
    Ok(()) // main function returns Result, so we return Ok(())
}

// example function using ? operator
fn process_user_data() -> Result<String, &'static str> {
    let raw_input = get_user_input()?; // if error, return early
    let validated = validate_input(&raw_input)?; // if error, return early
    let processed = process_input(&validated)?; // if error, return early
    
    Ok(format!("processed: {}", processed)) // all good, return success
}

// helper functions for the chain
fn get_user_input() -> Result<String, &'static str> {
    // simulate getting input that might fail
    Ok(String::from("42"))
}

fn validate_input(input: &str) -> Result<i32, &'static str> {
    input.parse::<i32>().map_err(|_| "invalid number format")
}

fn process_input(num: &i32) -> Result<i32, &'static str> {
    if *num > 0 {
        Ok(num * 2)
    } else {
        Err("number must be positive")
    }
}

// file operations with ? operator
fn calculate_from_files() -> Result<i32, io::Error> {
    let content1 = fs::read_to_string("number1.txt")?; // propagate io::Error
    let content2 = fs::read_to_string("number2.txt")?; // propagate io::Error
    
    // parse numbers (note: we need to handle parse errors differently)
    let num1 = content1.trim().parse::<i32>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "invalid number in file1")
    })?;
    
    let num2 = content2.trim().parse::<i32>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "invalid number in file2")
    })?;
    
    Ok(num1 + num2)
}

// function that loads config (for main function example)
fn load_config() -> Result<String, &'static str> {
    // simulate loading config
    Ok(String::from("app_config.json"))
}

// function with mixed error types
fn mixed_error_function() -> Result<String, Box<dyn std::error::Error>> {
    // different error types, but ? handles the conversion automatically
    let file_content = fs::read_to_string("config.txt")?; // io::Error
    let number: i32 = file_content.trim().parse()?; // ParseIntError
    
    Ok(format!("parsed number: {}", number))
}

// comparison: with and without ? operator
fn comparison_example() {
    println!("\n=== comparison: with vs without ? ===");
    
    // without ? operator (verbose)
    match multiply_numbers_verbose("5", "10") {
        Ok(result) => println!("verbose result: {}", result),
        Err(e) => println!("verbose error: {}", e),
    }
    
    // with ? operator (clean)
    match multiply_numbers_clean("5", "10") {
        Ok(result) => println!("clean result: {}", result),
        Err(e) => println!("clean error: {}", e),
    }
}

// without ? operator - lots of boilerplate
fn multiply_numbers_verbose(a: &str, b: &str) -> Result<i32, &'static str> {
    let num_a = match a.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err("failed to parse first number"),
    };
    
    let num_b = match b.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err("failed to parse second number"),
    };
    
    let result = match multiply_if_positive(num_a, num_b) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    
    Ok(result)
}

// with ? operator - much cleaner
fn multiply_numbers_clean(a: &str, b: &str) -> Result<i32, &'static str> {
    let num_a = a.parse::<i32>().map_err(|_| "failed to parse first number")?;
    let num_b = b.parse::<i32>().map_err(|_| "failed to parse second number")?;
    let result = multiply_if_positive(num_a, num_b)?;
    
    Ok(result)
}

fn multiply_if_positive(a: i32, b: i32) -> Result<i32, &'static str> {
    if a > 0 && b > 0 {
        Ok(a * b)
    } else {
        Err("both numbers must be positive")
    }
}

/*
what we learned about the ? operator:

the ? operator is syntactic sugar for:
match result {
    Ok(value) => value,
    Err(e) => return Err(e.into()), // note the .into() for type conversion
}

key features:
1. early return: if result is err, immediately return that error
2. unwrapping: if result is ok, extract the value and continue
3. type conversion: automatically converts error types using into() trait
4. works with result<t, e> and option<t>

where you can use ?:
- inside functions that return result<t, e> or option<t>
- in main() function if main returns result
- chains multiple operations cleanly

? vs other approaches:
- ? is much cleaner than nested match statements
- ? is safer than unwrap() because it doesn't panic
- ? propagates errors instead of handling them locally

common pattern:
fn do_complex_task() -> Result<String, SomeError> {
    let step1 = operation1()?; // if fails, return error immediately
    let step2 = operation2(&step1)?; // if fails, return error immediately
    let step3 = operation3(&step2)?; // if fails, return error immediately
    
    Ok(format!("all steps completed: {}", step3)) // success case
}

the ? operator makes rust error handling:
- concise (no boilerplate)
- readable (linear flow)
- safe (no panics)
- composable (easy to chain operations)
*/
