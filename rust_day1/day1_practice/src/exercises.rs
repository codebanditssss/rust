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

fn main() {
    let kiki = "42";
    let kiki: i32 = kiki.parse().unwrap(); 
    let kiki = kiki + 8; 
    println!("Kiki is {}", kiki);
}
