// result vs option comparison
// when to use Result<T, E> vs Option<T>

fn main() {
    println!("=== result vs option comparison ===\n");
    
    // key difference: Option for "absence", Result for "failure"
    
    println!("1. option<t> examples (absence of value):");
    option_examples();
    
    println!("\n2. result<t, e> examples (operations that can fail):");
    result_examples();
    
    println!("\n3. side by side comparison:");
    side_by_side_comparison();
    
    println!("\n4. conversion between option and result:");
    conversion_examples();
    
    println!("\n5. real world decision guide:");
    decision_guide_examples();
}

// option<t> examples - for values that may or may not exist
fn option_examples() {
    // finding item in collection - might not exist
    let numbers = vec![1, 2, 3, 4, 5];
    let found = find_number(&numbers, 3);  // returns Option<usize>
    match found {
        Some(index) => println!("  found number at index: {}", index),
        None => println!("  number not found"),
    }
    
    // accessing array element - might be out of bounds
    let item = numbers.get(10);  // returns Option<&i32>
    match item {
        Some(value) => println!("  element at index 10: {}", value),
        None => println!("  index 10 is out of bounds"),
    }
    
    // parsing that might not make sense
    let maybe_first_char = get_first_char("");  // returns Option<char>
    match maybe_first_char {
        Some(ch) => println!("  first character: {}", ch),
        None => println!("  string is empty, no first character"),
    }
    
    // optional configuration values
    let config_value = get_optional_setting("theme");  // returns Option<String>
    match config_value {
        Some(theme) => println!("  theme setting: {}", theme),
        None => println!("  no theme configured, using default"),
    }
}

// result<t, e> examples - for operations that can fail with reasons
fn result_examples() {
    // file operations - can fail for many reasons
    let file_content = read_config_file("missing.txt");  // returns Result<String, io::Error>
    match file_content {
        Ok(content) => println!("  file content: {}", content),
        Err(e) => println!("  file read failed: {}", e),
    }
    
    // parsing user input - can fail with parse error
    let parsed_age = parse_user_age("not_a_number");  // returns Result<u32, ParseError>
    match parsed_age {
        Ok(age) => println!("  parsed age: {}", age),
        Err(e) => println!("  age parsing failed: {}", e),
    }
    
    // network operations - can fail with connection errors
    let api_response = fetch_user_data("invalid_id");  // returns Result<String, NetworkError>
    match api_response {
        Ok(data) => println!("  api response: {}", data),
        Err(e) => println!("  api call failed: {}", e),
    }
    
    // validation - can fail with specific validation errors
    let validated_email = validate_email("invalid.email");  // returns Result<String, ValidationError>
    match validated_email {
        Ok(email) => println!("  valid email: {}", email),
        Err(e) => println!("  email validation failed: {}", e),
    }
}

// side by side comparison of similar operations
fn side_by_side_comparison() {
    println!("  searching for user:");
    
    // option version - user might not exist (normal situation)
    let user_maybe = find_user_by_name("kiki");  // Option<User>
    match user_maybe {
        Some(user) => println!("    option: found user {}", user.name),
        None => println!("    option: user not found (normal situation)"),
    }
    
    // result version - user lookup might fail (error situation)
    let user_result = lookup_user_in_database("kiki");  // Result<User, DatabaseError>
    match user_result {
        Ok(user) => println!("    result: found user {}", user.name),
        Err(e) => println!("    result: database lookup failed: {}", e),
    }
    
    println!("\n  getting configuration value:");
    
    // option version - setting might not be configured (normal)
    let setting_maybe = get_config_setting("max_connections");  // Option<String>
    match setting_maybe {
        Some(value) => println!("    option: setting value: {}", value),
        None => println!("    option: setting not configured, using default"),
    }
    
    // result version - reading config might fail (error)
    let setting_result = read_config_setting("max_connections");  // Result<String, ConfigError>
    match setting_result {
        Ok(value) => println!("    result: setting value: {}", value),
        Err(e) => println!("    result: config read failed: {}", e),
    }
}

// conversion between option and result
fn conversion_examples() {
    println!("  converting option to result:");
    
    // option to result with ok_or
    let maybe_value: Option<i32> = None;
    let result_value: Result<i32, &str> = maybe_value.ok_or("value is missing");
    println!("    option -> result: {:?}", result_value);
    
    // option to result with ok_or_else
    let maybe_name: Option<String> = None;
    let result_name: Result<String, String> = maybe_name.ok_or_else(|| {
        "name was not provided by user".to_string()
    });
    println!("    option -> result with closure: {:?}", result_name);
    
    println!("\n  converting result to option:");
    
    // result to option with ok() - loses error information
    let result_age: Result<u32, &str> = Err("invalid age format");
    let option_age: Option<u32> = result_age.ok();
    println!("    result -> option: {:?}", option_age);
    
    // result to option with err() - gets just the error
    let result_data: Result<String, &str> = Err("network timeout");
    let option_error: Option<&str> = result_data.err();
    println!("    result error -> option: {:?}", option_error);
}

// real world decision guide
fn decision_guide_examples() {
    println!("  use option<t> when:");
    println!("    - value might legitimately not exist");
    println!("    - absence is a normal, expected situation");
    println!("    - you don't need to know WHY it's missing");
    
    // examples of option usage
    let empty_list: Vec<i32> = vec![];  // specify type for empty vector
    let first_item = get_first_item(&empty_list);  // empty list is normal
    let user_preference = get_user_preference("theme");  // not set is normal
    let search_result = find_in_list(&vec![1, 2, 3], 5);  // not found is normal
    
    println!("\n  use result<t, e> when:");
    println!("    - operation can fail for specific reasons");
    println!("    - you need to know WHY it failed");
    println!("    - failure is an error condition to handle");
    
    // examples of result usage
    let _file_data = try_read_file("config.txt");  // file ops can fail
    let _parsed_int = try_parse_number("abc");     // parsing can fail
    let _network_response = try_fetch_data("url"); // network can fail
    let _validated_input = try_validate_email("test"); // validation can fail
    
    println!("\n  decision flowchart:");
    println!("    can the operation fail? -> yes: use result<t, e>");
    println!("                           -> no: does value exist? -> maybe: use option<t>");
    println!("                                                    -> always: use t directly");
}

// helper functions for examples

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// option examples - absence is normal
fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    // find index of target number, returns None if not found
    numbers.iter().position(|&x| x == target)
}

fn get_first_char(s: &str) -> Option<char> {
    // get first character, returns None if string is empty
    s.chars().next()
}

fn get_optional_setting(key: &str) -> Option<String> {
    // simulate optional configuration
    match key {
        "theme" => Some("dark".to_string()),
        _ => None,  // setting not configured
    }
}

fn find_user_by_name(name: &str) -> Option<User> {
    // simulate user search - user might not exist (normal)
    match name {
        "kiki" => Some(User { name: name.to_string(), age: 25 }),
        _ => None,  // user doesn't exist
    }
}

fn get_config_setting(key: &str) -> Option<String> {
    // simulate getting optional config value
    match key {
        "max_connections" => Some("100".to_string()),
        _ => None,  // setting not configured
    }
}

fn get_first_item<T>(items: &[T]) -> Option<&T> {
    // get first item from slice, None if empty
    items.first()
}

fn get_user_preference(key: &str) -> Option<String> {
    // user preferences might not be set
    match key {
        "theme" => Some("light".to_string()),
        _ => None,
    }
}

fn find_in_list(items: &[i32], target: i32) -> Option<usize> {
    // find item in list, None if not found
    items.iter().position(|&x| x == target)
}

// result examples - failure needs explanation

use std::fs;
use std::io;

fn read_config_file(filename: &str) -> Result<String, io::Error> {
    // file operations can fail for many reasons
    fs::read_to_string(filename)
}

#[derive(Debug)]
struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

fn parse_user_age(input: &str) -> Result<u32, ParseError> {
    // parsing can fail with specific error
    input.parse().map_err(|_| ParseError(format!("'{}' is not a valid age", input)))
}

#[derive(Debug)]
struct NetworkError(String);

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "network error: {}", self.0)
    }
}

impl std::error::Error for NetworkError {}

fn fetch_user_data(user_id: &str) -> Result<String, NetworkError> {
    // simulate network operation that can fail
    if user_id == "invalid_id" {
        Err(NetworkError("user not found on server".to_string()))
    } else {
        Ok(format!("user data for {}", user_id))
    }
}

#[derive(Debug)]
struct ValidationError(String);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "validation error: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}

fn validate_email(email: &str) -> Result<String, ValidationError> {
    // email validation can fail with specific reasons
    if !email.contains('@') {
        Err(ValidationError("email must contain @ symbol".to_string()))
    } else if !email.contains('.') {
        Err(ValidationError("email must contain domain".to_string()))
    } else {
        Ok(email.to_string())
    }
}

#[derive(Debug)]
struct DatabaseError(String);

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "database error: {}", self.0)
    }
}

impl std::error::Error for DatabaseError {}

fn lookup_user_in_database(name: &str) -> Result<User, DatabaseError> {
    // database operations can fail
    match name {
        "kiki" => Ok(User { name: name.to_string(), age: 25 }),
        _ => Err(DatabaseError("connection timeout".to_string())),
    }
}

#[derive(Debug)]
struct ConfigError(String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "config error: {}", self.0)
    }
}

impl std::error::Error for ConfigError {}

fn read_config_setting(key: &str) -> Result<String, ConfigError> {
    // reading config can fail
    match key {
        "max_connections" => Ok("100".to_string()),
        _ => Err(ConfigError("config file corrupted".to_string())),
    }
}

// more examples for decision guide
fn try_read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn try_parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}

fn try_fetch_data(url: &str) -> Result<String, NetworkError> {
    if url.is_empty() {
        Err(NetworkError("empty url".to_string()))
    } else {
        Ok("response data".to_string())
    }
}

fn try_validate_email(email: &str) -> Result<String, ValidationError> {
    validate_email(email)
}

/*
what we learned about result vs option:

when to use option<t>:
- value might legitimately not exist
- absence is normal, expected situation
- you don't need to know WHY it's missing
- examples: finding item in list, getting optional config, first/last elements

when to use result<t, e>:
- operation can fail for specific reasons
- you need to know WHY it failed
- failure is an error condition to handle
- examples: file operations, parsing, network calls, validation

key differences:
option<t>:
- some(t) or none
- represents absence vs presence
- none is not an error, just "nothing there"
- use when missing is normal

result<t, e>:
- ok(t) or err(e)
- represents success vs failure
- err contains information about what went wrong
- use when failure needs explanation

conversion methods:
option -> result:
- option.ok_or(error) -> converts none to err(error)
- option.ok_or_else(|| error) -> computes error with closure

result -> option:
- result.ok() -> converts to option, loses error info
- result.err() -> gets just the error part

decision flowchart:
1. can operation fail? -> yes: use result<t, e>
2. if no, does value always exist? -> no: use option<t>
3. if yes: use t directly

both option and result:
- work with ? operator
- have map, and_then, unwrap_or methods
- are monads (can chain operations)
- provide safe null/error handling
*/
