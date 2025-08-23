// Result<T, E> Basics
// error handling without exceptions - rust's approach to safe error management

// result is an enum similar to option, but for operations that can fail
// enum Result<T, E> {
//     Ok(T),   // success case with value of type T
//     Err(E),  // error case with error of type E
// }

fn main() {
    // creating result values
    let success: Result<i32, String> = Ok(42); // successful result with value 42
    let failure: Result<i32, String> = Err("something went wrong".to_string()); // error with message
    
    println!("created result values");
    
    // handling results with match
    match success {
        Ok(value) => println!("success: got value {}", value), // extract the success value
        Err(error) => println!("error: {}", error),
    }
    
    match failure {
        Ok(value) => println!("success: got value {}", value),
        Err(error) => println!("error: {}", error), // this will run
    }
    
    // practical example: safe division
    let result1 = safe_divide(10, 2); // returns Ok(5)
    let result2 = safe_divide(10, 0); // returns Err("division by zero")
    
    handle_division_result(result1);
    handle_division_result(result2);
    
    // example: parsing numbers safely
    let valid_number = parse_integer("42"); // returns Ok(42)
    let invalid_number = parse_integer("abc"); // returns Err("invalid number format")
    
    match valid_number {
        Ok(num) => println!("parsed number: {}", num),
        Err(error) => println!("parsing failed: {}", error),
    }
    
    match invalid_number {
        Ok(num) => println!("parsed number: {}", num),
        Err(error) => println!("parsing failed: {}", error), // this runs
    }
    
    // example: checking user age validity
    let ages = vec!["25", "abc", "150", "30"];
    
    for age_str in ages {
        let result = validate_age(age_str);
        match result {
            Ok(age) => println!("valid age: {}", age),
            Err(error) => println!("invalid age '{}': {}", age_str, error),
        }
    }
    
    // result with different error types
    let file_result: Result<String, std::io::Error> = read_config_file("config.txt");
    match file_result {
        Ok(content) => println!("file content: {}", content),
        Err(io_error) => println!("file error: {}", io_error),
    }
    
    // comparing with option
    demonstrate_result_vs_option();
}

// safe division that returns result instead of panicking
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string()) // return error case
    } else {
        Ok(a / b) // return success case
    }
}

fn handle_division_result(result: Result<i32, String>) {
    match result {
        Ok(value) => println!("division result: {}", value),
        Err(error) => println!("division error: {}", error),
    }
}

// parse string to integer with error message
fn parse_integer(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(number) => Ok(number), // parsing succeeded
        Err(_) => Err("invalid number format".to_string()), // parsing failed
    }
}

// validate age with multiple error conditions
fn validate_age(age_str: &str) -> Result<u32, String> {
    // first try to parse as number
    let age = match age_str.parse::<u32>() {
        Ok(num) => num,
        Err(_) => return Err("not a valid number".to_string()),
    };
    
    // then validate the range
    if age == 0 {
        Err("age cannot be zero".to_string())
    } else if age > 120 {
        Err("age too high (max 120)".to_string())
    } else {
        Ok(age)
    }
}

// simulate reading a config file (will fail since file doesn't exist)
fn read_config_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename) // this returns Result<String, std::io::Error>
}

fn demonstrate_result_vs_option() {
    // option: when something might not exist
    fn find_user_by_id(id: u32) -> Option<String> {
        if id == 1 {
            Some("alice".to_string()) // found user
        } else {
            None // user doesn't exist
        }
    }
    
    // result: when something can fail with specific error
    fn get_user_by_id(id: u32) -> Result<String, String> {
        if id == 0 {
            Err("invalid id: cannot be zero".to_string()) // specific error
        } else if id == 1 {
            Ok("kiki".to_string()) // success
        } else {
            Err("user not found".to_string()) // different specific error
        }
    }
    
    // option usage
    match find_user_by_id(5) {
        Some(user) => println!("found user: {}", user),
        None => println!("user not found"), // no details about why
    }
    
    // result usage
    match get_user_by_id(5) {
        Ok(user) => println!("got user: {}", user),
        Err(error) => println!("error getting user: {}", error), // specific error info
    }
}

/*
result<t, e> fundamentals:

problem with exceptions in other languages:
- functions don't declare what errors they can throw
- easy to forget to handle errors
- performance overhead from unwinding stack
- can crash program if not caught

rust's solution with result:
- errors are part of the function's type signature
- compiler forces you to handle both success and error cases
- zero runtime overhead
- no hidden control flow

basic structure:
- Ok(value) when operation succeeds
- Err(error) when operation fails
- both are explicit and must be handled

key differences from option:
- option: value might not exist (None)
- result: operation might fail with details (Err(reason))

when to use result:
- file operations (might fail: permission, not found, etc.)
- network operations (might fail: timeout, connection, etc.)
- parsing operations (might fail: invalid format)
- validation operations (might fail: business rules)
- any operation where you need to know WHY it failed

when to use option:
- optional configuration values
- searching collections (found/not found)
- optional struct fields
- any case where absence is normal and doesn't need explanation

common result types:
- Result<T, String> for simple error messages
- Result<T, std::io::Error> for file operations
- Result<T, std::num::ParseIntError> for number parsing
- Result<T, CustomError> for your own error types

mental model:
- result forces you to think about failure cases upfront
- makes error handling explicit and visible
- prevents forgetting to handle errors
- makes code more reliable and predictable
*/
