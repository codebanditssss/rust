// exercise 1 — name and favorite number

use std::io;

fn main() {
    let mut name = String::new();
    println!("enter ur name:");
    io::stdin().read_line(&mut name).expect("failed to read line");

    let mut fav_no = String::new();
    println!("enter ur favorite number:");
    io::stdin().read_line(&mut fav_no).expect("failed to read line");

    let fav_no: i32 = fav_no.trim().parse().unwrap();
    println!("hello, {}! ur favorite number is {}.", name.trim(), fav_no);
}