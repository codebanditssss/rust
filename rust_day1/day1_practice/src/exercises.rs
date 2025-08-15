//Exercise 1 — Mutable vs Immutable

//Question:
//Make a variable score = 10. Increase it by 5. Print the result.

// fn main(){
//     let mut score = 22;
//     score = score + 5;
//     println!("Score is {}", score);
// }

//Exercise 2 — Shadowing

//Question:
//Use shadowing to convert a string "42" to an integer and add 8.   

// fn main() {
//     let kiki = "42";
//     let kiki: i32 = kiki.parse().unwrap(); 
//     let kiki = kiki + 8; 
//     println!("Kiki is {}", kiki);
// }


//Exercise 3 — Constants
// Question:
// Declare a constant MAX_USERS = 100 and print it.

// fn main() {
//     const MAX_USERS: i32 = 100;
//     println!("Max users = {}", MAX_USERS);
// }

// Exercise 4 — Multiple Variables

// Question:
// Declare three variables: name as string, age as integer, score as float. Print them in one sentence.

fn main() {
    let name = "aash";
    let age: i32 = 21;
    let score: f32 = 22.0;
    println!("my name is {}, i am {} years old and my score is {}", name, age, score);
}