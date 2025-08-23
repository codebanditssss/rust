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
// fn square(x: i32) -> i32 {
//     x * x  // implicit return
// }

// fn main() {
//     let result = square(22);
//     println!("square = {}", result);
// }

// no semicolon ; after x * x
// if you add ;, it becomes a statement and returns nothing (()), leading to an error


// Explicit return
fn cube(x: i32) -> i32 {
    x * x * x
}

fn main() {
    let result = cube(6);
    println!("cube = {}", result);
}


// How Returning Works in Rust

// Statements vs Expressions
// Statements: Do something, but don’t return a value.
// let x = 5;   // statement

// Expressions: Produce a value.
// 5 + 3   // expression, evaluates to 8

// In Rust, functions return the value of the last expression inside them.



// Using Explicit return
// fn cube(x: i32) -> i32 {
//     return x * x * x;
// }

// return immediately exits the function.
// You can use it anywhere, not just at the end.
// This is handy when you want to exit early (like in conditionals).


// fn absolute_value(x: i32) -> i32 {
//     if x < 0 {
//         return -x;   // exit early
//     }
//     x  // last expression for non-negative
// }

//Implicit Return (Last Expression Style)

// fn cube(x: i32) -> i32 {
//     x * x * x  // no `return`, no semicolon
// }


// Cleaner, more idiomatic Rust.
// The last expression is automatically returned.
// No semicolon at the end, because ; turns an expression into a statement.

//so 

// fn cube(x: i32) -> i32 {
//     x * x * x;  //  This becomes a statement → returns `()`
// }

// will fail with a type mismatch.

//When to Prefer Each Style

// Use implicit style (last expression) → when function is short and has a single natural return value.

// fn square(x: i32) -> i32 {
//     x * x
// }

// Use explicit return →
// When you want to exit early (like guard clauses).
// When clarity is more important than brevity.

//Example with early exit:

// fn divide(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         return 0;  // graceful handling of division by zero
//     }
//     a / b
// }



// Last expression → returned automatically.
// If you add ;, it stops being an expression, so nothing is returned.
// Explicit return is optional, but idiomatic Rust avoids it unless there’s a good reason (like early exit).


// Rust encourages expression-based returns.
// Explicit return is there when you need to exit early or for clarity.
// Forgetting and putting a semicolon at the end is the #1 mistake beginners make.
