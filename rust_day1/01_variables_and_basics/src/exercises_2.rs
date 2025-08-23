// exercise 1 — name and favorite number

// use std::io;

// fn main() {
//     let mut name = String::new();
//     println!("enter ur name:");
//     io::stdin().read_line(&mut name).expect("failed to read line");

//     let mut fav_no = String::new();
//     println!("enter ur favorite number:");
//     io::stdin().read_line(&mut fav_no).expect("failed to read line");

//     let fav_no: i32 = fav_no.trim().parse().unwrap();
//     println!("hello, {}! ur favorite number is {}.", name.trim(), fav_no);
// }

// exercise 2 — sum of two numbers

// use std::io;

// fn main() {
//     let mut kiki = String::new();
//     let mut aash = String::new();

//     println!("enter first number:");
//     io::stdin().read_line(&mut kiki).expect("failed to read line");

//     println!("enter second number:");
//     io::stdin().read_line(&mut aash).expect("failed to read line");

//     let kiki: i32 = kiki.trim().parse().unwrap();
//     let aash: i32 = aash.trim().parse().unwrap();

//     let sum = kiki + aash;
//     println!("the sum of {} and {} is {}", kiki, aash, sum);
// }

// exercise 3 — multiply user input by 2

use std::io;

fn main() {
    let mut input = String::new();
    println!("enter a number");
    io::stdin().read_line(&mut input).expect("failed to read line");

    let input: i32 = input.trim().parse().unwrap();
    let result = input * 2;

    println!("the result of multiplying {} by 2 is {}", input, result);

    // .trim() removes leading and trailing whitespace, including the newline \n that comes when you press enter
    // so input = "22\n" becomes "22" before parsing -> works fine
    // without .trim()
    // input still contains the newline, eg "22\n"
    // .parse() tries to convert "22\n" -> integer -> fails
    // .unwrap() will panic, and the program crashes with an error
}