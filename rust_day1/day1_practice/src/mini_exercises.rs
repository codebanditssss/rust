//Exercise 1 — Convert height to cm and calculate BMI

use std::io;

fn main() {
    let mut height = String::new();
    println!("enter ur height in meters");
    io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().unwrap();

    let heightcm = height * 100.0;
    let weight = 44.0;
    let bmi = weight / (height * height);

    println!("ur height in cm is: {}", heightcm);
    println!("ur BMI is: {}", bmi);
}