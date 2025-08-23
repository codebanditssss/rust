// file operations & real world examples
// applying result handling to actual file i/o and data processing

use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== file operations & real world examples ===\n");
    
    // create some test files for our examples
    setup_test_files()?;
    
    // example 1: reading and processing config files
    println!("1. config file processing:");
    match process_config_file("config.txt") {
        Ok(config) => println!("loaded config: {:?}", config),
        Err(e) => println!("config error: {}", e),
    }
    
    // example 2: reading numbers from file and calculating
    println!("\n2. data file processing:");
    match calculate_from_data_file("numbers.txt") {
        Ok(result) => println!("calculation result: {}", result),
        Err(e) => println!("calculation error: {}", e),
    }
    
    // example 3: log file analysis
    println!("\n3. log file analysis:");
    match analyze_log_file("app.log") {
        Ok(stats) => println!("log analysis: {:?}", stats),
        Err(e) => println!("log analysis error: {}", e),
    }
    
    // example 4: backup and restore operations
    println!("\n4. backup operations:");
    match backup_important_files() {
        Ok(count) => println!("backed up {} files", count),
        Err(e) => println!("backup error: {}", e),
    }
    
    // example 5: user data processing pipeline
    println!("\n5. data processing pipeline:");
    match process_user_data_pipeline("users.csv") {
        Ok(summary) => println!("processing complete: {}", summary),
        Err(e) => println!("pipeline error: {}", e),
    }
    
    // cleanup test files
    cleanup_test_files();
    
    Ok(())
}

// config file structure
#[derive(Debug)]
struct AppConfig {
    app_name: String,
    version: String,
    debug_mode: bool,
    max_connections: u32,
}

// log statistics
#[derive(Debug)]
struct LogStats {
    total_lines: usize,
    error_count: usize,
    warning_count: usize,
    info_count: usize,
}

// example 1: config file processing with validation
fn process_config_file(filename: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    // fs::read_to_string reads entire file into memory as String
    // ? operator propagates io::Error if file doesn't exist or can't be read
    let content = fs::read_to_string(filename)?;
    
    // initialize variables to store parsed config values
    // using mut because we'll modify them in the loop
    let mut app_name = String::new();
    let mut version = String::new();
    let mut debug_mode = false;
    let mut max_connections = 0;
    
    // content.lines() returns iterator over each line in the file
    for line in content.lines() {
        // line.trim() removes whitespace from start and end
        let line = line.trim();
        
        // skip empty lines and comment lines (starting with #)
        if line.is_empty() || line.starts_with('#') {
            continue; // continue skips rest of loop iteration
        }
        
        // split line by '=' character to get key=value pairs
        // collect() converts iterator into Vec<&str>
        let parts: Vec<&str> = line.split('=').collect();
        
        // validate that we have exactly 2 parts (key and value)
        if parts.len() != 2 {
            // format! creates formatted string, .into() converts to Box<dyn Error>
            return Err(format!("invalid config line: {}", line).into());
        }
        
        // extract key and value, trimming whitespace
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        // match on key to determine which config field to set
        match key {
            "app_name" => app_name = value.to_string(), // convert &str to String
            "version" => version = value.to_string(),
            // value.parse()? converts string to bool, ? propagates parse error
            "debug_mode" => debug_mode = value.parse()?,
            // value.parse()? converts string to u32, ? propagates parse error
            "max_connections" => max_connections = value.parse()?,
            // _ is catch-all pattern for unknown keys
            _ => return Err(format!("unknown config key: {}", key).into()),
        }
    }
    
    // validate that required fields were found and set
    if app_name.is_empty() {
        return Err("app_name is required".into());
    }
    if version.is_empty() {
        return Err("version is required".into());
    }
    
    // construct and return AppConfig struct
    Ok(AppConfig {
        app_name,
        version,
        debug_mode,
        max_connections,
    })
}

// example 2: reading numbers and performing calculations
fn calculate_from_data_file(filename: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // read entire file content as string, ? propagates io errors
    let content = fs::read_to_string(filename)?;
    
    // create empty vector to store parsed numbers
    let mut numbers = Vec::new();
    
    // iterate through each line in the file
    for line in content.lines() {
        // remove whitespace from line
        let line = line.trim();
        
        // only process non-empty lines
        if !line.is_empty() {
            // parse string to f64 (64-bit floating point)
            // ? propagates ParseFloatError if string can't be parsed as number
            let number: f64 = line.parse()?;
            
            // add parsed number to our vector
            numbers.push(number);
        }
    }
    
    // check if we found any valid numbers
    if numbers.is_empty() {
        return Err("no valid numbers found in file".into());
    }
    
    // calculate average of all numbers
    // numbers.iter().sum() adds all elements in the vector
    let sum: f64 = numbers.iter().sum();
    
    // convert vector length to f64 for division
    // numbers.len() returns usize, we cast to f64 with 'as f64'
    let average = sum / numbers.len() as f64;
    
    // return calculated average wrapped in Ok()
    Ok(average)
}

// example 3: log file analysis
fn analyze_log_file(filename: &str) -> Result<LogStats, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    
    let mut stats = LogStats {
        total_lines: 0,
        error_count: 0,
        warning_count: 0,
        info_count: 0,
    };
    
    for line in content.lines() {
        stats.total_lines += 1;
        
        let line_lower = line.to_lowercase();
        if line_lower.contains("error") {
            stats.error_count += 1;
        } else if line_lower.contains("warning") || line_lower.contains("warn") {
            stats.warning_count += 1;
        } else if line_lower.contains("info") {
            stats.info_count += 1;
        }
    }
    
    Ok(stats)
}

// example 4: backup operations with multiple file handling
fn backup_important_files() -> Result<usize, Box<dyn std::error::Error>> {
    let files_to_backup = ["config.txt", "numbers.txt", "app.log"];
    let backup_dir = "backup";
    
    // create backup directory if it doesn't exist
    if !Path::new(backup_dir).exists() {
        fs::create_dir(backup_dir)?;
    }
    
    let mut backup_count = 0;
    
    for filename in &files_to_backup {
        if Path::new(filename).exists() {
            // read original file
            let content = fs::read_to_string(filename)?;
            
            // write to backup location
            let backup_path = format!("{}/{}.backup", backup_dir, filename);
            fs::write(&backup_path, content)?;
            
            backup_count += 1;
            println!("  backed up: {} -> {}", filename, backup_path);
        }
    }
    
    Ok(backup_count)
}

// example 5: complex data processing pipeline
fn process_user_data_pipeline(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    // step 1: read and validate csv file
    let content = fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return Err("file is empty".into());
    }
    
    // step 2: parse header
    let header = lines[0];
    let expected_columns = ["name", "age", "email"];
    let actual_columns: Vec<&str> = header.split(',').map(|s| s.trim()).collect();
    
    for expected in &expected_columns {
        if !actual_columns.contains(expected) {
            return Err(format!("missing required column: {}", expected).into());
        }
    }
    
    // step 3: process data rows
    let mut valid_users = 0;
    let mut total_age = 0;
    let mut email_domains = std::collections::HashSet::new();
    
    for (line_num, line) in lines.iter().skip(1).enumerate() {
        let row_num = line_num + 2; // +2 because we skipped header and arrays are 0-indexed
        
        match process_user_row(line, row_num) {
            Ok((age, domain)) => {
                valid_users += 1;
                total_age += age;
                email_domains.insert(domain);
            }
            Err(e) => {
                println!("  skipping row {}: {}", row_num, e);
            }
        }
    }
    
    if valid_users == 0 {
        return Err("no valid user records found".into());
    }
    
    // step 4: generate summary
    let avg_age = total_age / valid_users;
    let summary = format!(
        "processed {} users, average age: {}, unique domains: {}",
        valid_users,
        avg_age,
        email_domains.len()
    );
    
    // step 5: save summary to file
    fs::write("processing_summary.txt", &summary)?;
    
    Ok(summary)
}

// helper function for processing individual user rows
fn process_user_row(line: &str, row_num: usize) -> Result<(u32, String), Box<dyn std::error::Error>> {
    let fields: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    
    if fields.len() != 3 {
        return Err(format!("expected 3 fields, got {}", fields.len()).into());
    }
    
    let name = fields[0];
    let age_str = fields[1];
    let email = fields[2];
    
    // validate name
    if name.is_empty() {
        return Err("name cannot be empty".into());
    }
    
    // validate and parse age
    let age: u32 = age_str.parse()
        .map_err(|_| format!("invalid age: {}", age_str))?;
    
    if age > 120 {
        return Err(format!("unrealistic age: {}", age).into());
    }
    
    // validate email and extract domain
    if !email.contains('@') {
        return Err(format!("invalid email: {}", email).into());
    }
    
    let domain = email.split('@').nth(1)
        .ok_or("email missing domain")?
        .to_string();
    
    Ok((age, domain))
}

// setup test files for our examples
fn setup_test_files() -> Result<(), Box<dyn std::error::Error>> {
    // create config.txt
    let config_content = r#"# application configuration
app_name=MyRustApp
version=1.2.3
debug_mode=true
max_connections=100
"#;
    fs::write("config.txt", config_content)?;
    
    // create numbers.txt
    let numbers_content = "42.5\n17.3\n89.1\n23.7\n56.9\n";
    fs::write("numbers.txt", numbers_content)?;
    
    // create app.log
    let log_content = r#"2024-01-15 10:00:00 INFO Application started
2024-01-15 10:01:15 INFO User connected: user123
2024-01-15 10:02:30 WARNING High memory usage detected
2024-01-15 10:03:45 ERROR Database connection failed
2024-01-15 10:04:00 INFO Retrying database connection
2024-01-15 10:04:15 INFO Database connection restored
2024-01-15 10:05:30 ERROR User authentication failed
"#;
    fs::write("app.log", log_content)?;
    
    // create users.csv with your names
    let csv_content = r#"name,age,email
kiki,28,kiki@example.com
anaya,35,anaya@company.org
aash,42,aash@university.edu
"#;
    fs::write("users.csv", csv_content)?;
    
    Ok(())
}

// cleanup test files
fn cleanup_test_files() {
    let files = ["config.txt", "numbers.txt", "app.log", "users.csv", "processing_summary.txt"];
    
    for file in &files {
        if Path::new(file).exists() {
            let _ = fs::remove_file(file); // ignore errors during cleanup
        }
    }
    
    // remove backup directory if it exists
    if Path::new("backup").exists() {
        let _ = fs::remove_dir_all("backup"); // ignore errors during cleanup
    }
}

/*
what we learned about real-world file operations:

common patterns:
1. read entire file with fs::read_to_string()
2. process line by line with content.lines()
3. parse and validate data with proper error handling
4. use ? operator to propagate file i/o errors
5. validate data format and content
6. handle missing files gracefully

error handling strategies:
- use fs::read_to_string()? for simple file reading
- convert parse errors with .map_err() for better messages
- validate business logic and return custom errors
- use Box<dyn std::error::Error> for mixed error types
- create backup operations with proper error propagation

real-world scenarios covered:
- configuration file parsing
- data file processing and calculations
- log file analysis and statistics
- backup operations with multiple files
- csv data processing pipeline

best practices:
- always validate file format before processing
- provide meaningful error messages
- handle edge cases (empty files, malformed data)
- use structured data types for complex configs
- clean up temporary files when appropriate
- use ? operator for clean error propagation
*/
