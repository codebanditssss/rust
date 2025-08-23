// Enum Methods with impl blocks
// Adding behavior to enums just like structs

// Define our enums first
enum TrafficLight {
    Red,
    Yellow, 
    Green,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// Implement methods for TrafficLight
impl TrafficLight {
    // Method that returns how long to wait (in seconds)
    fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
    
    // Method that returns next light in sequence
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    // Method that checks if it's safe to go
    fn can_go(&self) -> bool {
        match self {
            TrafficLight::Red => false,
            TrafficLight::Yellow => false,
            TrafficLight::Green => true,
        }
    }
    
    // Associated function (like a constructor)
    fn new_red() -> TrafficLight {
        TrafficLight::Red
    }
    
    // Associated function to create from string
    fn from_string(color: &str) -> Option<TrafficLight> {
        match color.to_lowercase().as_str() {
            "red" => Some(TrafficLight::Red),
            "yellow" => Some(TrafficLight::Yellow),
            "green" => Some(TrafficLight::Green),
            _ => None,
        }
    }
}

// Implement methods for Message
impl Message {
    // Check if message contains quit command
    fn is_quit(&self) -> bool {
        match self {
            Message::Quit => true,
            _ => false,
        }
    }
    
    // Get a description of the message
    fn description(&self) -> String {
        match self {
            Message::Quit => "Quit message".to_string(),
            Message::Move { x, y } => format!("Move to ({}, {})", x, y),
            Message::Write(text) => format!("Write: {}", text),
            Message::ChangeColor(r, g, b) => format!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
    
    // Process the message (performs the action)
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("Shutting down...");
            },
            Message::Move { x, y } => {
                println!("Moving cursor to position ({}, {})", x, y);
            },
            Message::Write(text) => {
                println!("Displaying text: {}", text);
            },
            Message::ChangeColor(r, g, b) => {
                println!("Setting background color to RGB({}, {}, {})", r, g, b);
            },
        }
    }
    
    // Constructor functions
    fn new_quit() -> Message {
        Message::Quit
    }
    
    fn new_move(x: i32, y: i32) -> Message {
        Message::Move { x, y }
    }
    
    fn new_write(text: &str) -> Message {
        Message::Write(text.to_string())
    }
    
    fn new_color(r: u8, g: u8, b: u8) -> Message {
        Message::ChangeColor(r, g, b)
    }
}

// Implement methods for WebEvent
impl WebEvent {
    // Get event type as string
    fn event_type(&self) -> &str {
        match self {
            WebEvent::PageLoad => "page_load",
            WebEvent::KeyPress(_) => "key_press",
            WebEvent::Paste(_) => "paste",
            WebEvent::Click { .. } => "click",
        }
    }
    
    // Check if event involves text input
    fn involves_text(&self) -> bool {
        match self {
            WebEvent::KeyPress(_) | WebEvent::Paste(_) => true,
            _ => false,
        }
    }
    
    // Get position for events that have coordinates
    fn position(&self) -> Option<(i64, i64)> {
        match self {
            WebEvent::Click { x, y } => Some((*x, *y)),
            _ => None,
        }
    }
    
    // Log the event
    fn log(&self) {
        match self {
            WebEvent::PageLoad => println!("🌐 Page loaded"),
            WebEvent::KeyPress(key) => println!("⌨️  Key pressed: '{}'", key),
            WebEvent::Paste(text) => println!("📋 Text pasted: '{}'", text),
            WebEvent::Click { x, y } => println!("🖱️  Clicked at ({}, {})", x, y),
        }
    }
}

fn main() {
    // Using TrafficLight methods
    let light = TrafficLight::Red;
    println!("Current light wait time: {} seconds", light.wait_time());
    println!("Can go? {}", light.can_go());
    
    let next_light = light.next();
    println!("Next light wait time: {} seconds", next_light.wait_time());
    
    // Using associated functions (constructors)
    let red_light = TrafficLight::new_red();
    println!("Created red light, wait time: {}", red_light.wait_time());
    
    // Creating from string
    if let Some(green_light) = TrafficLight::from_string("green") {
        println!("Created green light from string: {}", green_light.can_go());
    }
    
    // Using Message methods
    let messages = vec![
        Message::new_quit(),
        Message::new_move(10, 20),
        Message::new_write("Hello, World!"),
        Message::new_color(255, 0, 128),
    ];
    
    for msg in &messages {
        println!("Message: {}", msg.description());
        println!("Is quit? {}", msg.is_quit());
        msg.process();
        println!("---");
    }
    
    // Using WebEvent methods
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('a'),
        WebEvent::Paste("Hello from clipboard".to_string()),
        WebEvent::Click { x: 100, y: 200 },
    ];
    
    for event in &events {
        println!("Event type: {}", event.event_type());
        println!("Involves text? {}", event.involves_text());
        
        if let Some((x, y)) = event.position() {
            println!("Position: ({}, {})", x, y);
        }
        
        event.log();
        println!("---");
    }
}

/*
What we built with enum methods:

PROBLEM: Enums are just data, but often you want to DO things with that data.
Like: get next traffic light, check if message is quit, log web events, etc.

SOLUTION: impl blocks let you add methods to enums, just like structs.

What methods we added:

1. Instance methods (&self):
   - light.wait_time() - get behavior based on current variant
   - msg.is_quit() - check properties of the current variant  
   - event.log() - perform actions based on current variant
   - All use match internally to handle different variants

2. Associated functions (no &self):
   - TrafficLight::new_red() - constructor functions
   - Message::new_write("text") - convenient builders
   - TrafficLight::from_string("red") - parsing/conversion

3. Pattern-based methods:
   - next() - transform one variant to another
   - description() - convert variant to human-readable string
   - position() - extract data only from certain variants (returns Option)

Key insights:

- Enums + impl = data + behavior together
- Methods use match to handle different variants appropriately  
- Associated functions work like constructors or utilities
- Some methods return Option when not all variants have the data
- You can have both data (enum variants) and behavior (methods) in one type

Real world examples:
- HTTP Status codes with .is_success(), .is_error() methods
- Game states with .update(), .render() methods  
- File types with .can_execute(), .icon() methods
- Database results with .is_ok(), .error_message() methods

This makes enums much more powerful than just "tagged unions" - 
they become full-featured types with both data and behavior.

Common patterns:
✅ Use match inside methods to handle all variants
✅ Return Option from methods that don't apply to all variants
✅ Create associated functions for common constructors
✅ Add convenience methods for common checks (is_xyz())
✅ Add transform methods (next(), to_string(), etc.)
*/
