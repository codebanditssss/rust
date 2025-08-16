//Exercise 1 — Convert height to cm and calculate BMI

// use std::io;

// fn main() {
//     let mut height = String::new();
//     println!("enter ur height in meters");
//     io::stdin().read_line(&mut height).expect("failed to read line");
//     let height: f32 = height.trim().parse().unwrap();

//     let heightcm = height * 100.0;
//     let weight = 44.0;
//     let bmi = weight / (height * height);

//     println!("ur height in cm is: {}", heightcm);
//     println!("ur BMI is: {}", bmi);
// }

// Exercise 2 — Two favorite numbers, do arithmetic

use std::io;

fn main() {
    let mut kiki = String::new();
    let mut aash = String::new();

    println!("enter ur first fav number");
    io::stdin().read_line(&mut kiki).expect("failed to read line");

    println!("enter ur second fav number");
    io::stdin().read_line(&mut aash).expect("failed to read line");

    let kiki: i32 = kiki.trim().parse().unwrap();
    let aash: i32 = aash.trim().parse().unwrap();   

    let sum = kiki + aash;
    let diff = kiki - aash;
    let prod = kiki * aash;
    let div = kiki / aash;

    
    println!("sum: {}", sum);
    println!("difference: {}", diff);
    println!("product: {}", prod);
    println!("division: {}", div);

}


// io::stdin().read_line(&mut kiki)
// small note


// in rust, we use &mut when we want to borrow a value mutably
// we can modify the borrowed value
// only one mutable reference can exist at a time (to prevent data races)
// read_line needs to append the user’s input into the input string
// that means it must change the string
// to allow a function to modify a variable without taking ownership
// we pass a mutable reference (&mut input)
// if we used just &input
// it would be an immutable reference, so the function wouldn’t be allowed to modify the string -> compiler error
// if we didn’t use & at all
// we had be passing ownership of the variable to the function
// and after the call, we couldn’t use input anymore
// &mut is the safe middle ground -> let the function change the value but still keep ownership with us