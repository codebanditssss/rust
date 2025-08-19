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


fn greet() {
    println!("hello from a function!");
}

fn main() {
    greet(); // function call
}

// fn greet() → takes no arguments.
// Inside main, greet(); calls it.
// Rust programs always start at main(), so other functions must be called from there.