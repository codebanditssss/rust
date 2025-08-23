// custom error types and conversion
// building your own error enums for better error handling

use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== custom error types ===\n");
    
    // example 1: basic custom error enum
    println!("1. basic custom error:");
    match validate_user_age("25") {
        Ok(age) => println!("valid age: {}", age),
        Err(e) => println!("age error: {}", e),
    }
    
    match validate_user_age("150") {
        Ok(age) => println!("valid age: {}", age),
        Err(e) => println!("age error: {}", e),
    }
    
    // example 2: file processing with custom errors
    println!("\n2. file processing with custom errors:");
    match process_config_file("config.txt") {
        Ok(config) => println!("config loaded: {:?}", config),
        Err(e) => println!("config error: {}", e),
    }
    
    // example 3: user registration with multiple error types
    println!("\n3. user registration:");
    match register_user("kiki", "17", "kiki@email") {
        Ok(user) => println!("user registered: {:?}", user),
        Err(e) => {
            println!("registration failed: {}", e);
            // show how to handle specific error types
            match e {
                UserError::InvalidAge(_) => println!("  -> fix: provide valid age"),
                UserError::InvalidEmail(_) => println!("  -> fix: provide valid email"),
                UserError::NameTooShort => println!("  -> fix: name must be longer"),
                UserError::DatabaseError(_) => println!("  -> fix: try again later"),
            }
        }
    }
    
    // example 4: error conversion and chaining
    println!("\n4. error conversion:");
    match process_data_file("numbers.txt") {
        Ok(result) => println!("data processed: {}", result),
        Err(e) => {
            println!("processing failed: {}", e);
            // show the error chain
            println!("error type: {:?}", e);
        }
    }
    
    Ok(())
}

// define custom error enum for age validation
#[derive(Debug)]
enum AgeError {
    InvalidFormat,     // string can't be parsed as number
    TooYoung,         // age less than 0
    TooOld,           // age greater than 120
    Unrealistic,      // specific unrealistic ages
}

// implement Display trait so error can be printed
impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::InvalidFormat => write!(f, "age must be a valid number"),
            AgeError::TooYoung => write!(f, "age cannot be negative"),
            AgeError::TooOld => write!(f, "age cannot be greater than 120"),
            AgeError::Unrealistic => write!(f, "age seems unrealistic"),
        }
    }
}

// implement Error trait so it can be used in Result<T, E>
impl std::error::Error for AgeError {}

// function using custom error type
fn validate_user_age(age_str: &str) -> Result<u32, AgeError> {
    // try to parse string as number
    let age: u32 = age_str.parse().map_err(|_| AgeError::InvalidFormat)?;
    
    // validate age ranges with custom error variants
    if age > 120 {
        return Err(AgeError::TooOld);
    }
    
    if age == 0 {
        return Err(AgeError::Unrealistic);
    }
    
    Ok(age)
}

// more complex custom error for config file processing
#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),           // file doesn't exist, stores filename
    InvalidFormat(String),          // malformed config line, stores line content
    MissingRequired(String),        // required field missing, stores field name
    InvalidValue { key: String, value: String }, // invalid value for key
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(filename) => {
                write!(f, "config file not found: {}", filename)
            }
            ConfigError::InvalidFormat(line) => {
                write!(f, "invalid config format in line: '{}'", line)
            }
            ConfigError::MissingRequired(field) => {
                write!(f, "required config field missing: {}", field)
            }
            ConfigError::InvalidValue { key, value } => {
                write!(f, "invalid value '{}' for config key '{}'", value, key)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

// simple config struct
#[derive(Debug)]
struct AppConfig {
    name: String,
    version: String,
    debug: bool,
}

// function using custom config error
fn process_config_file(filename: &str) -> Result<AppConfig, ConfigError> {
    // create a simple config file for testing
    let config_content = "name=MyApp\nversion=1.0\ndebug=true\n";
    fs::write(filename, config_content).map_err(|_| {
        ConfigError::FileNotFound(filename.to_string())
    })?;
    
    // read and parse config file
    let content = fs::read_to_string(filename).map_err(|_| {
        ConfigError::FileNotFound(filename.to_string())
    })?;
    
    let mut name = None;
    let mut version = None;
    let mut debug = None;
    
    // parse each line
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // split by '=' character
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(ConfigError::InvalidFormat(line.to_string()));
        }
        
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        // parse each config key
        match key {
            "name" => name = Some(value.to_string()),
            "version" => version = Some(value.to_string()),
            "debug" => {
                debug = Some(value.parse().map_err(|_| {
                    ConfigError::InvalidValue {
                        key: key.to_string(),
                        value: value.to_string(),
                    }
                })?);
            }
            _ => {
                return Err(ConfigError::InvalidValue {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }
    }
    
    // check required fields
    let name = name.ok_or_else(|| ConfigError::MissingRequired("name".to_string()))?;
    let version = version.ok_or_else(|| ConfigError::MissingRequired("version".to_string()))?;
    let debug = debug.ok_or_else(|| ConfigError::MissingRequired("debug".to_string()))?;
    
    Ok(AppConfig { name, version, debug })
}

// comprehensive custom error for user registration
#[derive(Debug)]
enum UserError {
    InvalidAge(String),              // age validation failed, stores input
    InvalidEmail(String),            // email validation failed, stores input
    NameTooShort,                    // name is too short
    DatabaseError(io::Error),        // database operation failed, wraps io::Error
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::InvalidAge(age) => {
                write!(f, "invalid age '{}' - must be 18-65", age)
            }
            UserError::InvalidEmail(email) => {
                write!(f, "invalid email '{}' - must contain @ and domain", email)
            }
            UserError::NameTooShort => {
                write!(f, "name too short - must be at least 2 characters")
            }
            UserError::DatabaseError(e) => {
                write!(f, "database error: {}", e)
            }
        }
    }
}

impl std::error::Error for UserError {
    // implement source() to show error chain
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UserError::DatabaseError(e) => Some(e),
            _ => None,
        }
    }
}

// convert io::Error to UserError automatically
impl From<io::Error> for UserError {
    fn from(error: io::Error) -> Self {
        UserError::DatabaseError(error)
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// user registration function with custom error handling
fn register_user(name: &str, age_str: &str, email: &str) -> Result<User, UserError> {
    // validate name length
    if name.len() < 2 {
        return Err(UserError::NameTooShort);
    }
    
    // validate and parse age
    let age: u32 = age_str.parse().map_err(|_| {
        UserError::InvalidAge(age_str.to_string())
    })?;
    
    if age < 18 || age > 65 {
        return Err(UserError::InvalidAge(age_str.to_string()));
    }
    
    // validate email format
    if !email.contains('@') || !email.contains('.') {
        return Err(UserError::InvalidEmail(email.to_string()));
    }
    
    // simulate database save (could fail with io error)
    save_user_to_database(name, age, email)?; // ? automatically converts io::Error to UserError
    
    Ok(User {
        name: name.to_string(),
        age,
        email: email.to_string(),
    })
}

// simulate database operation that can fail
fn save_user_to_database(name: &str, age: u32, email: &str) -> Result<(), io::Error> {
    // simulate saving to file
    let user_data = format!("{},{},{}\n", name, age, email);
    fs::write("users_db.txt", user_data)?; // this could fail with io::Error
    Ok(())
}

// custom error that combines multiple error types
#[derive(Debug)]
enum DataProcessingError {
    IoError(io::Error),              // file operation failed
    ParseError(ParseIntError),       // number parsing failed
    ValidationError(String),         // custom validation failed
    EmptyFile,                       // file has no content
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataProcessingError::IoError(e) => write!(f, "file error: {}", e),
            DataProcessingError::ParseError(e) => write!(f, "parse error: {}", e),
            DataProcessingError::ValidationError(msg) => write!(f, "validation error: {}", msg),
            DataProcessingError::EmptyFile => write!(f, "file is empty"),
        }
    }
}

impl std::error::Error for DataProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DataProcessingError::IoError(e) => Some(e),
            DataProcessingError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

// automatic conversion from io::Error
impl From<io::Error> for DataProcessingError {
    fn from(error: io::Error) -> Self {
        DataProcessingError::IoError(error)
    }
}

// automatic conversion from ParseIntError
impl From<ParseIntError> for DataProcessingError {
    fn from(error: ParseIntError) -> Self {
        DataProcessingError::ParseError(error)
    }
}

// function that uses multiple error conversions
fn process_data_file(filename: &str) -> Result<f64, DataProcessingError> {
    // create test file
    fs::write(filename, "10\n20\n30\n40\n50\n")?; // io::Error auto-converts
    
    // read file content
    let content = fs::read_to_string(filename)?; // io::Error auto-converts
    
    // check if file is empty
    if content.trim().is_empty() {
        return Err(DataProcessingError::EmptyFile);
    }
    
    let mut numbers = Vec::new();
    
    // parse each line as number
    for line in content.lines() {
        let line = line.trim();
        if !line.is_empty() {
            let number: i32 = line.parse()?; // ParseIntError auto-converts
            
            // custom validation
            if number < 0 {
                return Err(DataProcessingError::ValidationError(
                    format!("negative number not allowed: {}", number)
                ));
            }
            
            numbers.push(number);
        }
    }
    
    // calculate average
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;
    
    Ok(average)
}

/*
what we learned about custom error types:

custom error enum structure:
#[derive(Debug)]
enum MyError {
    VariantOne,                    // simple variant
    VariantTwo(String),           // variant with data
    VariantThree { field: String }, // struct-like variant
}

required trait implementations:
1. fmt::Display - how error appears when printed
2. std::error::Error - marks it as an error type
3. Debug - automatic with #[derive(Debug)]

error conversion with From trait:
impl From<OtherError> for MyError {
    fn from(error: OtherError) -> Self {
        MyError::WrappedError(error)
    }
}

benefits of custom errors:
- specific error variants for different failure modes
- store relevant context data in error variants
- automatic error conversion with From trait
- better error messages for users
- type safety - compiler ensures all errors handled

patterns:
- use enum variants to categorize different error types
- store error context (filename, invalid value, etc.) in variants
- implement From<T> for automatic error conversion with ?
- use source() method to chain errors together
- provide helpful error messages in Display implementation

custom errors make error handling:
- more specific (know exactly what went wrong)
- more informative (store context about the failure)
- more composable (automatic conversion between error types)
- more maintainable (clear error categories)
*/
