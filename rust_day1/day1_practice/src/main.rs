// fn main() {
//     println!("Hello, khushi!"); //here println! is a macro, not a function (hence the !).
// }

// in rust a macro is code that writes code for u before the program is compiled
// it takes input and generates rust code at compile time 
// why is println! a macro? ...normal rust functions cant take “variable number of arguments” in the same flexible way (without extra boilerplate), but macros can


//-------------------

// step 2- adding placeholders

// fn main() {
//     let name = "ash";
//     println!("hello, {}!", name);
// }

// {} is replaced in order with variables and if you add more {} than variables then it gives compile error

//----------------------

//step 3- adding numbered placeholders

fn main() {
    println!("{0}, meet {1}. {1}, meet {0}.", "kiki", "ash");
}

// {0} and {1} let you reuse arguments without repeating