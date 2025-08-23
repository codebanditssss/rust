// Match Statement Fundamentals
// Pattern matching to handle different enum variants and extract their data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // this variant holds state data
}

#[derive(Debug)] // allows printing with {:?}
enum UsState {
    Alabama,
    Alaska,
    // other states...
}

fn main() {
    let msg = Message::Write(String::from("Hello from match!")); // create a Write message
    
    // match against each possible variant
    match msg {
        Message::Quit => {
            println!("Time to quit!");
        },
        Message::Move { x, y } => { // destructure x and y from struct variant
            println!("Moving to coordinates: x={}, y={}", x, y);
        },
        Message::Write(text) => { // extract the String from tuple variant
            println!("Writing message: {}", text);
        },
        Message::ChangeColor(r, g, b) => { // extract all three color values
            println!("Changing color to RGB({}, {}, {})", r, g, b);
        },
    }
    
    // call function with different message types
    process_message(Message::Quit);
    process_message(Message::Move { x: 10, y: 20 });
    process_message(Message::ChangeColor(255, 0, 0));
    
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    let light = TrafficLight::Red; // set current light state
    
    // match without storing return value
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
    
    // store the match result in a variable
    let action = match TrafficLight::Green {
        TrafficLight::Red => "stop",
        TrafficLight::Yellow => "slow down", 
        TrafficLight::Green => "go",
    };
    println!("Action: {}", action);
    
    let number = 3; // can match on integers too
    match number {
        1 => println!("One"),
        2 => println!("Two"), 
        3 => println!("Three"),
        _ => println!("Something else"), // handles 4, 5, 6, etc.
    }
    
    // using the Coin enum defined at the top
    
    let coin = Coin::Quarter(UsState::Alabama); // create quarter from Alabama
    let value = value_in_cents(coin); // get its cent value
    println!("Coin value: {} cents", value);
}

fn process_message(msg: Message) {
    match msg { // handle each message type
        Message::Quit => {
            println!("Received quit message - shutting down");
        },
        Message::Move { x, y } => { // get x and y coordinates
            println!("Moving cursor to position ({}, {})", x, y);
        },
        Message::Write(s) => { // get the text content
            println!("Text message: {}", s);
        },
        Message::ChangeColor(red, green, blue) => { // get RGB values
            println!("Setting color to RGB({}, {}, {})", red, green, blue);
        },
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // return cent value based on coin type
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // extract the state from quarter
            println!("Quarter from {:?}!", state); // use the state data
            25 // return quarter's value
        },
    }
}

/*
Key match features:
- Exhaustive: must handle all variants
- Pattern destructuring: extract data from variants  
- Expression: returns a value
- Use _ for catch-all (must be last)
- Variable binding: name the extracted data

Common mistakes:
❌ match msg { Message::Quit = println!("quit") }     // wrong: use => not =
❌ match light { Red => "stop" }                      // wrong: need TrafficLight::Red
❌ match coin { Penny => 1, Nickel => 5 }             // wrong: missing other variants
❌ match num { _ => "other", 1 => "one" }             // wrong: _ must be last
❌ match msg { Message::Write{text} => ... }          // wrong: use (text) not {text}

✅ match msg { Message::Quit => println!("quit") }    // correct: use =>
✅ match light { TrafficLight::Red => "stop" }        // correct: full path
✅ match coin { Penny => 1, _ => 0 }                  // correct: handle all cases
✅ match num { 1 => "one", _ => "other" }             // correct: _ comes last
✅ match msg { Message::Write(text) => ... }          // correct: tuple syntax
*/
