// result methods and safe handling examples
// all the ways to extract values from Result<T, E> safely

fn main() {
    println!("=== result methods & safe handling ===\n");
    
    // create some results to work with
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    
    // 1. unwrap() - panics on error (dangerous!)
    println!("1. unwrap() method:");
    println!("success.unwrap() = {}", success.unwrap());
    // println!("failure.unwrap() = {}", failure.unwrap()); // this would panic!
    
    // 2. expect() - panics with custom message (slightly better)
    println!("\n2. expect() method:");
    println!("success.expect(\"should work\") = {}", success.expect("should work"));
    // println!("failure.expect(\"this should work!\") = {}", failure.expect("this should work!")); // panics with custom message
    
    // 3. unwrap_or() - provides default value on error (safe!)
    println!("\n3. unwrap_or() method:");
    println!("success.unwrap_or(0) = {}", success.unwrap_or(0));
    println!("failure.unwrap_or(0) = {}", failure.unwrap_or(0)); // returns 0 instead of panicking
    
    // 4. unwrap_or_else() - calls closure to compute default (safe!)
    println!("\n4. unwrap_or_else() method:");
    println!("success.unwrap_or_else(|err| compute_default) = {}", 
             success.unwrap_or_else(|err| { println!("error: {}", err); -1 }));
    println!("failure.unwrap_or_else(|err| compute_default) = {}", 
             failure.unwrap_or_else(|err| { println!("error: {}", err); -1 }));
    
    // 5. is_ok() and is_err() - check without consuming (safe!)
    println!("\n5. is_ok() and is_err() methods:");
    println!("success.is_ok() = {}", success.is_ok());
    println!("success.is_err() = {}", success.is_err());
    println!("failure.is_ok() = {}", failure.is_ok());
    println!("failure.is_err() = {}", failure.is_err());
    
    // 6. ok() - converts Result<T, E> to Option<T> (ignores error)
    println!("\n6. ok() method:");
    println!("success.ok() = {:?}", success.ok()); // Some(42)
    println!("failure.ok() = {:?}", failure.ok()); // None
    
    // 7. err() - converts Result<T, E> to Option<E> (ignores success)
    println!("\n7. err() method:");
    println!("success.err() = {:?}", success.err()); // None
    println!("failure.err() = {:?}", failure.err()); // Some("something went wrong")
    
    // 8. map() - transform the success value
    println!("\n8. map() method:");
    let doubled = success.map(|x| x * 2);
    let failed_doubled = failure.map(|x| x * 2);
    println!("success.map(|x| x * 2) = {:?}", doubled);
    println!("failure.map(|x| x * 2) = {:?}", failed_doubled); // error preserved
    
    // 9. map_err() - transform the error value
    println!("\n9. map_err() method:");
    let better_error = failure.map_err(|err| format!("detailed error: {}", err));
    println!("failure.map_err(|err| format!(\"detailed error: {{}}\", err)) = {:?}", better_error);
    
    // 10. and_then() - chain operations that can fail
    println!("\n10. and_then() method:");
    let chained = success.and_then(|x| {
        if x > 40 {
            Ok(x + 10) // another successful result
        } else {
            Err("value too small")
        }
    });
    println!("success.and_then(validate_and_add) = {:?}", chained);
    
    // real world examples
    println!("\n=== real world examples ===");
    
    // parsing with safe handling
    safe_parsing_example();
    
    // file operations with graceful errors
    file_operation_example();
    
    // chaining multiple operations
    operation_chain_example();
}

// example: safe string parsing
fn safe_parsing_example() {
    println!("\nsafe parsing example:");
    
    let inputs = ["42", "hello", "-17", "999"];
    
    for input in inputs {
        let result = input.parse::<i32>();
        
        // method 1: using unwrap_or with default
        let value1 = result.unwrap_or(-1);
        println!("'{}' -> unwrap_or(-1) = {}", input, value1);
        
        // method 2: using match for detailed handling
        let result = input.parse::<i32>(); // parse again since result was consumed
        match result {
            Ok(num) => println!("'{}' -> parsed successfully: {}", input, num),
            Err(e) => println!("'{}' -> parse failed: {}", input, e),
        }
    }
}

// example: file operations with graceful error handling
fn file_operation_example() {
    use std::fs;
    
    println!("\nfile operation example:");
    
    // try to read a file that might not exist
    let file_result = fs::read_to_string("maybe_exists.txt");
    
    // safe handling - provide default content
    let content = file_result.unwrap_or_else(|err| {
        println!("file read failed: {}", err);
        String::from("default content")
    });
    
    println!("file content (or default): '{}'", content);
    
    // alternative: check first, then decide
    let file_result = fs::read_to_string("maybe_exists.txt");
    if file_result.is_ok() {
        println!("file read successfully!");
    } else {
        println!("file read failed, using fallback");
    }
}

// example: chaining operations that can fail
fn operation_chain_example() {
    println!("\noperation chain example:");
    
    // simulate a chain of operations that can fail
    let result = parse_number("42")
        .and_then(|num| validate_positive(num))
        .and_then(|num| calculate_square_root(num))
        .map(|root| format!("square root: {:.2}", root));
    
    match result {
        Ok(message) => println!("chain succeeded: {}", message),
        Err(e) => println!("chain failed: {}", e),
    }
    
    // try with invalid input
    let result2 = parse_number("hello")
        .and_then(|num| validate_positive(num))
        .and_then(|num| calculate_square_root(num))
        .map(|root| format!("square root: {:.2}", root));
    
    match result2 {
        Ok(message) => println!("chain succeeded: {}", message),
        Err(e) => println!("chain failed: {}", e),
    }
}

// helper functions for chaining example
fn parse_number(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| format!("'{}' is not a number", s))
}

fn validate_positive(num: f64) -> Result<f64, String> {
    if num >= 0.0 {
        Ok(num)
    } else {
        Err(format!("{} is negative", num))
    }
}

fn calculate_square_root(num: f64) -> Result<f64, String> {
    Ok(num.sqrt())
}

/*
what we learned about result methods:

dangerous methods (can panic):
- unwrap() -> panics if error, returns value if ok
- expect("message") -> panics with custom message if error

safe methods (never panic):
- unwrap_or(default) -> returns default value if error
- unwrap_or_else(|err| compute_default) -> calls closure to compute default if error
- is_ok() -> true if result is ok, false if error (doesn't consume)
- is_err() -> true if result is error, false if ok (doesn't consume)

conversion methods:
- ok() -> converts result<t, e> to option<t> (throws away error info)
- err() -> converts result<t, e> to option<e> (throws away success info)

transformation methods:
- map(|value| transform) -> transforms success value, leaves error unchanged
- map_err(|error| transform) -> transforms error value, leaves success unchanged
- and_then(|value| another_result) -> chains operations that can fail

general pattern:
1. use is_ok()/is_err() to check without consuming the result
2. use unwrap_or/unwrap_or_else for safe defaults
3. use match for detailed error handling
4. use map/and_then for transforming and chaining
5. avoid unwrap/expect unless you're 100% sure it won't fail

result methods give you fine-grained control over error handling
this prevents crashes and makes your programs more robust
*/
