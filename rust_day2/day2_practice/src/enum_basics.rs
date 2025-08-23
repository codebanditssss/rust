// Enum Basics & Definition
// Learning: Basic enum syntax, creating instances, why enums > constants

// What is an enum?
// An enum (enumeration) lets you define a type that can be one of several possible variants
// Think of it as "this value can be A OR B OR C, but only one at a time"

// Example 1: Traffic Light (Basic Enum)
enum TrafficLight {
    Red,
    Yellow, 
    Green,
}

// TrafficLight is the enum name
// Red, Yellow, Green are the variants (possible values)
// Each variant is like a "choice" or "state" the enum can be in

fn main() {
    // Creating enum instances using ::
    let light1 = TrafficLight::Red;
    let light2 = TrafficLight::Yellow;
    let light3 = TrafficLight::Green;
    
    // The :: syntax means "access the variant Red from the TrafficLight enum"
    // This is similar to accessing associated functions like String::from()
    
    println!("Created three different traffic light states!");
    
    // You can store enums in variables, pass them to functions, etc.
    let current_light = TrafficLight::Red;
    check_light_action(current_light);
}

// Function that takes an enum as parameter
fn check_light_action(light: TrafficLight) {
    // We'll learn how to actually inspect the enum value in the next section (match)
    // For now, just know that we can pass enums around like any other value
    println!("Checking what action to take for this light...");
}

// Why use enums instead of constants?

// Bad approach with constants:
// const RED: i32 = 0;
// const YELLOW: i32 = 1;  
// const GREEN: i32 = 2;
// 
// Problems:
// - You could accidentally use any integer (what does 5 mean?)
// - No type safety (could pass wrong numbers)
// - No clear grouping of related values

// Good approach with enums:
// - TrafficLight can ONLY be Red, Yellow, or Green
// - Compiler prevents invalid states
// - Clear, readable code
// - Type safety guaranteed

// Example 2: Day of Week
enum DayOfWeek {
    Monday,
    Tuesday, 
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Example 3: Game State
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

// Key Points:
// 1. enum keyword to define
// 2. Variants separated by commas
// 3. Use EnumName::Variant to create instances
// 4. Much safer than constants or strings
// 5. Compiler ensures you handle all possible cases

// Common Beginner Mistakes:
// ❌ let light = Red;           // Missing TrafficLight::
// ❌ let light = "Red";         // Using strings instead of enums
// ❌ let light = TrafficLight.Red; // Using dot instead of ::

// ✅ let light = TrafficLight::Red; // Correct!
