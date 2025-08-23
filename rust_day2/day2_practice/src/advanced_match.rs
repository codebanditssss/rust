// Advanced Match Patterns and Guard Clauses
// More powerful pattern matching techniques

fn main() {
    // Range patterns
    let number = 42;
    
    match number {
        1..=10 => println!("Small number: {}", number), // 1 to 10 inclusive
        11..=50 => println!("Medium number: {}", number), // 11 to 50 inclusive  
        51..=100 => println!("Large number: {}", number),
        _ => println!("Very large number: {}", number),
    }
    
    // Character ranges
    let letter = 'g';
    match letter {
        'a'..='f' => println!("Early alphabet"),
        'g'..='m' => println!("Middle alphabet"), // this matches
        'n'..='z' => println!("Late alphabet"),
        _ => println!("Not a lowercase letter"),
    }
    
    // Multiple patterns with |
    let day_number = 6;
    match day_number {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"), // this matches (Saturday)
        _ => println!("Invalid day"),
    }
    
    // Guard clauses with if
    let num = 15;
    let is_even = num % 2 == 0;
    
    match num {
        x if x < 10 => println!("Small number: {}", x),
        x if x % 2 == 0 => println!("Even number: {}", x),
        x if x % 3 == 0 => println!("Divisible by 3: {}", x), // this matches (15)
        _ => println!("Other number: {}", num),
    }
    
    // Guards with Option
    let maybe_number = Some(7);
    
    match maybe_number {
        Some(x) if x > 10 => println!("Big number: {}", x),
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Small odd number: {}", x), // this matches
        None => println!("No number"),
    }
    
    // Destructuring structs in match
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 3, y: 4 };
    
    match point {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x: 0, y } => println!("On Y-axis at {}", y),
        Point { x, y: 0 } => println!("On X-axis at {}", x),
        Point { x, y } if x == y => println!("Diagonal point ({}, {})", x, y),
        Point { x, y } => println!("Point at ({}, {})", x, y), // this matches
    }
    
    // Destructuring with ignore patterns
    let tuple = (1, 2, 3, 4, 5);
    
    match tuple {
        (first, _, _, _, last) => { // ignore middle three values
            println!("First: {}, Last: {}", first, last);
        }
    }
    
    // Nested destructuring
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
    }
    
    let msg = Message::ChangeColor(Color::Rgb(255, 0, 128));
    
    match msg {
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(Color::Red) => println!("Change to red"),
        Message::ChangeColor(Color::Green) => println!("Change to green"),
        Message::ChangeColor(Color::Blue) => println!("Change to blue"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => { // nested destructuring
            println!("Change to RGB({}, {}, {})", r, g, b); // this matches
        }
    }
    
    // @ bindings (capture the whole value while destructuring)
    let value = Some(42);
    
    match value {
        Some(x @ 40..=50) => { // capture x AND check if it's in range
            println!("Found number in range 40-50: {}", x); // this matches
        },
        Some(x) => println!("Found other number: {}", x),
        None => println!("No value"),
    }
    
    // Complex @ binding example
    #[derive(Debug)] // allows printing with {:?}
    enum Request {
        Get { url: String },
        Post { url: String, data: String },
    }
    
    let request = Request::Post { 
        url: "api.example.com".to_string(), 
        data: "user_id=123".to_string() 
    };
    
    match request {
        Request::Get { url } => println!("GET request to {}", url),
        Request::Post { ref url, ref data } if url.contains("api") => { // use ref to borrow
            println!("API POST request to {} with data: {}", url, data);
        },
        Request::Post { url, data } => {
            println!("POST to {} with data: {}", url, data);
        }
    }
    
    // Match with references
    let numbers = vec![1, 2, 3, 4, 5];
    
    for number in &numbers { // iterate by reference
        match number {
            1 => println!("Found one"),
            2 | 3 => println!("Found two or three"),
            x if *x > 3 => println!("Found big number: {}", x), // dereference with *
            _ => println!("Found other"),
        }
    }
    
    // Matching on references vs values
    let opt_ref = &Some(42);
    
    match opt_ref {
        Some(x) => println!("Reference to Some: {}", x), // automatically dereferenced
        None => println!("Reference to None"),
    }
    
    match *opt_ref { // manually dereference
        Some(x) => println!("Dereferenced Some: {}", x),
        None => println!("Dereferenced None"),
    }
}

/*
Advanced pattern matching recap:

1. Range patterns: 1..=10, 'a'..='z'
   - Inclusive ranges with ..=
   - Works with numbers and chars
   - More readable than multiple | patterns for ranges

2. Multiple patterns: 1 | 2 | 3
   - Use | to match multiple values in one arm
   - Good for grouping related cases

3. Guard clauses: Some(x) if x > 10
   - Add extra conditions with 'if'
   - Can access matched variables in the condition
   - Evaluated after pattern matching succeeds

4. Destructuring: Point { x, y }, (first, _, last)
   - Extract fields from structs and tuples
   - Use _ to ignore values you don't need
   - Works with nested structures

5. @ bindings: x @ 1..=10
   - Capture the matched value while also testing it
   - Useful when you need both the pattern test and the value
   - Can be combined with guards

6. Reference patterns: &Some(x), *value
   - Match against references directly
   - Rust often auto-dereferences for you
   - Use * when you need explicit dereferencing

Common use cases:
- Range matching for categorizing numbers/chars
- Guards for complex conditions that can't be expressed in patterns
- Destructuring for extracting data from complex types
- @ bindings when you need both the test and the value
- Multiple patterns for grouping similar cases

Why these matter:
- Make complex matching logic clean and readable
- Reduce nested if-else chains
- Express intent more clearly than boolean conditions
- Leverage Rust's pattern matching power fully
*/
