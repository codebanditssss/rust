// Enum Variants with Data
// Learning: Storing different types of data in enum variants

// So far we've seen simple enums like TrafficLight::Red
// But enums become REALLY powerful when they can store data!

// Example 1: Message enum with different data types
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct-like data
    Write(String),              // Tuple-like data  
    ChangeColor(i32, i32, i32), // Multiple values in tuple
}

// Each variant can store different types and amounts of data:
// - Quit: stores nothing (like our TrafficLight variants)
// - Move: stores named fields (like a mini-struct)
// - Write: stores one String value
// - ChangeColor: stores three i32 values (RGB colors)

fn main() {
    // Creating instances with data
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 128);
    
    println!("Created 4 different message types!");
    
    // We can store these in a vector since they're all the same type (Message)
    let messages = vec![
        Message::Quit,
        Message::Move { x: 5, y: 15 },
        Message::Write(String::from("Learning enums")),
        Message::ChangeColor(0, 255, 0),
    ];
    
    println!("Created a vector with {} messages", messages.len());
    
    // Example 2: IpAddr enum (practical example)
    enum IpAddr {
        V4(u8, u8, u8, u8),     // IPv4: four numbers
        V6(String),             // IPv6: one string
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Example 3: Coin enum (from Rust book)
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),  // Quarter stores which state it's from
    }
    
    enum UsState {
        Alabama,
        Alaska,
        // ... (normally you'd list all 50 states)
    }
    
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);
    
    // Example 4: Different ways to store data
    enum WebEvent {
        // Unit variant (no data)
        PageLoad,
        
        // Tuple variants (unnamed fields)
        KeyPress(char),
        Paste(String),
        
        // Struct variant (named fields)
        Click { x: i64, y: i64 },
        
        // Multiple unnamed fields
        Resize(i32, i32),
    }
    
    let click_event = WebEvent::Click { x: 100, y: 50 };
    let key_event = WebEvent::KeyPress('q');
    let paste_event = WebEvent::Paste(String::from("Hello"));
    
    println!("Created various web events!");
}

// Key Differences from Structs:
// 
// Instead of defining separate structs:
// struct QuitMessage; 
// struct MoveMessage { x: i32, y: i32 }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);
//
// We use ONE enum that can be any of these!
// This is much cleaner and expresses "this is ONE OF these types"

// When to use each variant style:

// 1. Unit variants: Message::Quit
//    - When you just need to signal "this thing happened"
//    - Like flags or simple states

// 2. Tuple variants: Message::Write(String)
//    - When you have 1-3 values that don't need names
//    - Data is simple and order is obvious

// 3. Struct variants: Message::Move { x: i32, y: i32 }
//    - When you have multiple fields that benefit from names
//    - When clarity is more important than brevity
//    - Similar to defining a mini-struct inside the enum

// Memory efficiency:
// Rust stores enum variants efficiently - only uses as much memory
// as the largest variant + a small "tag" to know which variant it is

// Common mistakes:
// ❌ Message::Write{"hello"}      // Wrong syntax for tuple variant
// ❌ Message::Move(x: 10, y: 20)  // Wrong syntax for struct variant  
// ❌ Message::Move{10, 20}        // Mixed up tuple and struct syntax

// ✅ Message::Write(String::from("hello"))  // Correct tuple syntax
// ✅ Message::Move { x: 10, y: 20 }         // Correct struct syntax
