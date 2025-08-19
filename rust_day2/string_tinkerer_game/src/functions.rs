// function basics 

// functions let you break down code into smaller, reusable pieces
// instead of writing everything inside main(), you can create clear chunks of logic


// rust requires function definitions to appear before or after main() — both work, but order doesn’t matter because the compiler scans the whole file

// the general structure

// fn function_name(parameter_list) -> ReturnType {
    // function body
// }

// fn → keyword to define a function.
// function_name → by convention, snake_case in Rust (add_numbers, calculate_area).
// parameter_list → variables with types.
// -> ReturnType → only needed if the function returns something.
// Function body → curly braces { }.

// Example 1: Function with No Parameters & No Return


// fn greet() {
//     println!("hello from a function!");
// }

// fn main() {
//     greet(); // function call
// }

// fn greet() → takes no arguments.
// Inside main, greet(); calls it.
// Rust programs always start at main(), so other functions must be called from there.


// Example 2: Function with Parameters (but No Return)

// fn print_number(num: i32) {
//     println!("The number is: {}", num);
// }

// fn main() {
//     print_number(22);
// }

// num: i32 means this function expects an integer parameter.
// Rust is strictly typed, so you must specify parameter type.
// Calling without passing the correct type → compiler error.


// Example 3: Function with Parameters & Return Type

// fn add(x: i32, y: i32) -> i32 {
//     x + y  // last expression returned implicitly
// }

// fn main() {
//     let sum = add(22, 25);
//     println!("The sum is: {}", sum);
// }

// (x: i32, y: i32) are typed parameters.
// -> i32 means return type is integer.
// The last expression without ; becomes the return value.
// You could also write return x + y;, but idiomatic Rust avoids unnecessary return

// Forgetting to specify a type:
// fn print(num) { ... }  // Error, must declare type
// Adding ; after the last line in a returning function:
// fn add(x: i32, y: i32) -> i32 {
//     x + y; // This makes it return ()
// }

//Confusing fn my_fun() {} with fn my_fun() -> i32 {} — one returns nothing, the other must return an integer.

// ---------------------------------------------------------------------------------------------


// Returning Values & Expressions

/// Function That Returns a Value

//Rust requires you to specify the return type (if it isn’t () a.k.a unit).
fn square(x: i32) -> i32 {
    x * x  // implicit return
}

fn main() {
    let result = square(22);
    println!("square = {}", result);
}

// no semicolon ; after x * x
// if you add ;, it becomes a statement and returns nothing (()), leading to an error