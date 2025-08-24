use std::io;

// enum for all calculator operations
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// custom error type for calculator errors
#[derive(Debug)]
enum CalcError {
    InvalidOperation,
    InvalidNumber,
    DivisionByZero,
    ParseError(String),
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcError::InvalidOperation => write!(f, "invalid operation selected"),
            CalcError::InvalidNumber => write!(f, "invalid number entered"),
            CalcError::DivisionByZero => write!(f, "cannot divide by zero"),
            CalcError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

impl std::error::Error for CalcError {}

// convert from string parse errors to our custom error
impl From<std::num::ParseFloatError> for CalcError {
    fn from(err: std::num::ParseFloatError) -> Self {
        CalcError::ParseError(err.to_string())
    }
}

// parse operation from user input
fn parse_operation(input: &str) -> Result<Operation, CalcError> {
    match input.trim() {
        "1" | "+" | "add" => Ok(Operation::Add),
        "2" | "-" | "subtract" => Ok(Operation::Subtract),
        "3" | "*" | "multiply" => Ok(Operation::Multiply),
        "4" | "/" | "divide" => Ok(Operation::Divide),
        _ => Err(CalcError::InvalidOperation),
    }
}

// parse number from user input with better error handling
fn parse_number(input: &str) -> Result<f64, CalcError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CalcError::InvalidNumber);
    }
    
    // try to parse as f64
    trimmed.parse::<f64>().map_err(CalcError::from)
}

// perform the actual calculation
fn calculate(op: Operation, a: f64, b: f64) -> Result<f64, CalcError> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
    }
}

// get user input with prompt
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input
}

// display the calculator menu
fn show_menu() {
    println!("\n=== rust calculator ===");
    println!("select operation:");
    println!("1. add (+)");
    println!("2. subtract (-)");
    println!("3. multiply (*)");
    println!("4. divide (/)");
    println!("5. exit");
    println!("enter choice (1-5):");
}

fn main() {
    println!("welcome to kiki's rust calculator!");
    
    loop {
        show_menu();
        
        // get operation choice
        let choice = get_input("");
        
        // check for exit
        if choice.trim() == "5" || choice.trim().to_lowercase() == "exit" {
            println!("thanks for using the calculator! goodbye!");
            break;
        }
        
        // parse the operation
        let operation = match parse_operation(&choice) {
            Ok(op) => op,
            Err(e) => {
                println!("error: {}", e);
                println!("please try again.");
                continue;
            }
        };
        
        // get first number
        let first_input = get_input("enter first number:");
        let first_num = match parse_number(&first_input) {
            Ok(num) => num,
            Err(e) => {
                println!("error: {}", e);
                println!("please try again.");
                continue;
            }
        };
        
        // get second number
        let second_input = get_input("enter second number:");
        let second_num = match parse_number(&second_input) {
            Ok(num) => num,
            Err(e) => {
                println!("error: {}", e);
                println!("please try again.");
                continue;
            }
        };
        
        // perform calculation
        match calculate(operation, first_num, second_num) {
            Ok(result) => {
                println!("\nresult: {} = {}", 
                    format_calculation(&choice, first_num, second_num), 
                    result
                );
            }
            Err(e) => {
                println!("calculation error: {}", e);
            }
        }
        
        // ask if user wants to continue
        println!("\npress enter to continue or type 'exit' to quit...");
        let continue_input = get_input("");
        if continue_input.trim().to_lowercase() == "exit" {
            println!("thanks for using the calculator! goodbye!");
            break;
        }
    }
}

// helper function to format the calculation display
fn format_calculation(op_choice: &str, a: f64, b: f64) -> String {
    let symbol = match op_choice.trim() {
        "1" | "add" => "+",
        "2" | "subtract" => "-", 
        "3" | "multiply" => "*",
        "4" | "divide" => "/",
        s if s == "+" || s == "-" || s == "*" || s == "/" => s,
        _ => "?",
    };
    
    format!("{} {} {}", a, symbol, b)
}