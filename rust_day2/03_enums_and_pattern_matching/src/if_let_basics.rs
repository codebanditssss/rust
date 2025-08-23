// if let Syntax - Shorthand for Simple Matches
// When you only care about one pattern, if let is cleaner than match

fn main() {
    // The verbose way with match
    let some_number = Some(42);
    
    match some_number {
        Some(value) => println!("Got number: {}", value),
        None => {} // do nothing for None - this feels wasteful
    }
    
    // The cleaner way with if let
    if let Some(value) = some_number {
        println!("Got number with if let: {}", value);
    } // automatically handles None by doing nothing
    
    // Example: processing optional user input
    let user_input: Option<String> = Some("hello world".to_string());
    
    // Old verbose way
    match user_input {
        Some(text) => {
            println!("User typed: {}", text);
            println!("Length: {}", text.len());
        },
        None => {} // empty - we don't care about None case
    }
    
    // Clean way with if let
    let user_input2: Option<String> = Some("rust is cool".to_string());
    if let Some(text) = user_input2 {
        println!("User typed: {}", text);
        println!("Length: {}", text.len());
    } // if None, just skip this block
    
    // if let with else (when you do want to handle None)
    let maybe_age: Option<i32> = None;
    
    if let Some(age) = maybe_age {
        println!("User is {} years old", age);
    } else {
        println!("Age not provided"); // handles None case
    }
    
    // Practical example: config values
    let config_timeout: Option<u32> = Some(30);
    
    if let Some(timeout) = config_timeout {
        println!("Setting timeout to {} seconds", timeout);
        // use the timeout value...
    } else {
        println!("Using default timeout");
        // use default timeout...
    }
    
    // if let works with any enum, not just Option
    enum Message {
        Text(String),
        Number(i32),
        Quit,
    }
    
    let msg = Message::Text("Hello from enum!".to_string());
    
    // Only handle Text messages, ignore others
    if let Message::Text(content) = msg {
        println!("Received text message: {}", content);
    } // Number and Quit messages are ignored
    
    // Multiple if let checks
    let messages = vec![
        Message::Text("First message".to_string()),
        Message::Number(42),
        Message::Quit,
        Message::Text("Last message".to_string()),
    ];
    
    for msg in messages {
        if let Message::Text(content) = msg {
            println!("Text: {}", content); // only prints text messages
        }
        // Numbers and Quit are silently skipped
    }
    
    // Nested Option example
    let nested: Option<Option<i32>> = Some(Some(10));
    
    // Extract the inner value if both Options are Some
    if let Some(inner_option) = nested {
        if let Some(value) = inner_option {
            println!("Nested value: {}", value);
        }
    }
    
    // Working with Result and if let
    let parse_result: Result<i32, _> = "123".parse();
    
    if let Ok(number) = parse_result {
        println!("Parsed successfully: {}", number);
    } // ignore Err case
    
    // Chain if let conditions
    let first: Option<i32> = Some(1);
    let second: Option<i32> = Some(2);
    
    if let Some(a) = first {
        if let Some(b) = second {
            println!("Sum: {}", a + b); // both were Some
        }
    }
    
    // Comparison: when to use if let vs match
    demonstrate_when_to_use_each();
}

fn demonstrate_when_to_use_each() {
    let value: Option<i32> = Some(42);
    
    // Use match when you need to handle all cases
    let result = match value {
        Some(n) => n * 2,
        None => 0, // must handle None to return a value
    };
    println!("Match result: {}", result);
    
    // Use if let when you only care about one case
    if let Some(n) = value {
        let doubled = n * 2;
        println!("if let result: {}", doubled);
    } // don't care about None case
    
    // Use match for complex pattern matching
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
    }
    
    let color = Color::Rgb(255, 0, 128);
    
    // match handles all variants
    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::Rgb(r, g, b) => println!("Custom color: {}, {}, {}", r, g, b),
    }
    
    // if let for just one variant we care about
    let color2 = Color::Rgb(100, 200, 50);
    if let Color::Rgb(red, green, blue) = color2 {
        println!("Only handling RGB: {}, {}, {}", red, green, blue);
    } // ignore Red, Green, Blue variants
}

/*
What if let actually does:

PROBLEM: Sometimes you only want to handle the Some case of Option, or one specific enum variant.
Writing a full match with empty None => {} feels verbose and wasteful.

SOLUTION: if let lets you pattern match against one specific case and ignore the rest.

Basic syntax:
if let PATTERN = VALUE {
    // code when pattern matches
} else {
    // code when pattern doesn't match (optional)
}

What we built:

1. Simple Option handling:
   - if let Some(x) = maybe_value { use_x() }
   - Much cleaner than match when you don't care about None

2. Enum variant filtering:
   - if let Message::Text(content) = msg { process_text(content) }
   - Ignore other variants without writing empty match arms

3. Config processing:
   - if let Some(timeout) = config.timeout { set_timeout(timeout) }
   - Only act when config value exists

4. Loop filtering:
   - for msg in messages { if let Message::Text(s) = msg { print(s) } }
   - Process only the variants you care about

When to use what:

Use if let when:
- You only care about one pattern (usually Some or one enum variant)
- You want to ignore other cases without handling them
- The code is simpler than a full match

Use match when:
- You need to handle multiple patterns
- You need to return a value (match is an expression)
- You want exhaustive checking of all cases
- You have complex pattern matching needs

Real world examples:
- if let Some(user) = logged_in_user { show_dashboard(user) }
- if let Ok(data) = network_response { process_data(data) }
- if let Message::UserMessage(text) = incoming { reply_to_user(text) }

The key insight: if let is syntactic sugar for simple matches.
It makes common patterns more readable and less verbose.

Common patterns:
✅ if let Some(x) = option { use_x() }              // handle Some, ignore None
✅ if let Ok(data) = result { process(data) }       // handle success, ignore error
✅ if let MyEnum::Variant(x) = value { handle(x) }  // handle one variant, ignore others
✅ if let Some(x) = opt { ... } else { fallback }  // handle both cases cleanly

Mistakes to avoid:
❌ if let Some(x) = opt == true { ... }             // wrong syntax
❌ if Some(x) = opt { ... }                         // missing 'let'
❌ using if let when you need all cases handled     // use match instead
*/
