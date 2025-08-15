// Goal
// Take multiple inputs from user: name, age, favorite number, height
// Perform simple math on numbers
// Use shadowing, parsing, mutability, and printing
// Combine all learned concepts in one interactive program

use std::io;

fn main() {
    let mut name = String::new();
    println!("enter ur name:");
    io::stdin().read_line(&mut name).expect("failed to read line");

    let mut age = String::new();
    println!("enter ur age:");
    io::stdin().read_line(&mut age).expect("failed to read line");
    let age: i32 = age.trim().parse().unwrap();

    let mut fav_number = String::new();
    println!("enter ur favorite number:");
    io::stdin().read_line(&mut fav_number).expect("failed to read line");
    let fav_number: i32 = fav_number.trim().parse().unwrap();

    let mut height = String::new();
    println!("enter ur height in meters:");
    io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().unwrap();

    println!(
        "hello, {}! u are {} years old, your favorite number is {}, and your height is {} meters.",
        name.trim(),
        age,
        fav_number,
        height
    );
}
