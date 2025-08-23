// if else in rust

// fn main() {
//     let kiki = 22;

//     if kiki > 0 {
//         println!("kiki is positive");
//     } else if kiki < 0 {
//         println!("kiki is negative");
//     } else {
//         println!("kiki is zero");
//     }
// }

// unlike python, rust needs boolean conditions (can’t use integers as truthy/falsy)
// in python, you can do things like
// if 5:  print("yo") 
// here, 5 is treated as truthy (non-zero -> True)
// but in rust, that doesn’t work. rust is strict
// if 5 {   // ERROR: expected `bool`, found integer
    //println!("yo");
//}
// rust will not convert numbers into true/false automatically
// we must write an actual boolean expression
// if 5 > 0    //  works


// in rust, if and else conditions must always evaluate to a bool (true or false), not an integer, string, or any other type

// even or odd

// fn main() {
//     let aash = 25;

//     if aash % 2 == 0 {
//         println!("aash is even");
//     } else {
//         println!("aash is odd");
//     }
// }

// nested conditions

// fn main() {
//     let anaya = 18;
    
//     if anaya >= 18 {
//         println!("anaya is an adult");
//         if anaya < 21 {
//             println!("adult but not 21 yet");
//         } else {
//             println!("adult and 21+");
//         }
//     } else {
//         println!("anaya is a minor");
//     }
// }

// using if as an expression
// in rust, if can return a value

fn main() {
    let is_day = false;

    let activity = if is_day {
        "work"
    } else {
        "sleep"
    };

    println!("activity: {}", activity);
}
