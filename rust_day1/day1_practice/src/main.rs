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

// fn main() {
//     println!("{0}, meet {1}. {1}, meet {0}.", "kiki", "ash");
// }

// {0} and {1} let you reuse arguments without repeating


//----------------------

//step 4- named placeholders

// fn main() {
//     println!("{first} and {second} are learning rust", first = "ash", second ="kiki");
// }

// good for readability


//----------------------

// step 5- formatting options

// fn main() {
//     println!("pi is roughly {pi:.3}", pi = 3.14159);
//     println!("{:.2}", "Hello"); //  can't format a string as float

//     // u expected this to fail because "Hello" is a string and :.2 looks like “round to 2 decimal places” (float formatting)
//     // but in rust’s println!, the meaning of .precision actually depends on the type you r formatting
// }

//-------------------------

// next part step 1- immutable variables

// fn main() {
//     let x = 22;
//     println!("x is {}", x);
//     x = 33; // this will give error because x is immutable by default
// }

//--------------------------

// step 2- Mutable Variables

// fn main() {
//     let x = 22;
//     let x = 2.5;
//     println!("x is {}", x);
// }

// here, i didnt use mut
// the compiler did not throw an error because this is shadowing, not mutation
// shadowing means you r allowed to declare a new variable with the same name. rust sees it as a brand new variable
// so the first x is shadowed by the second x, which is a new variable

// let mut x = 22;
// even though x was mutable initially, the moment you write let x = 2.5;
// you are shadowing, so the mut on the first x doesnt matter
// shadowing creates a new variable — it separate from the previous mut variable

// let x = 5;
// x = 10; //  error: cannot assign twice to immutable variable

// let mut x = 5;
// x = 2.5; // error: expected integer, found float


//--------------------------

// step 3- Constants

fn main() {
    const PI: f32 = 3.14;
    println!("PI = {}", PI);
}

// must declare type explicitly
// immutable, cannot reassign
// forgetting type - compile error
// trying mut const PI - invalid
