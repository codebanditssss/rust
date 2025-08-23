// string tinkerer = a mini-game/tool where you enter a word or sentence, and the program “plays” with it by doing operations like:


// counting letters
// reversing it
// changing it (like uppercase, lowercase)
// giving you back a “tinkered” (modified) version


use std::io;

fn main() {
    println!("welcome to string tinkerer");
    
    loop {
        println!("\ntype a word or sentence, and i will play with it");
        println!("(or type 'exit' to quit the game)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let input = input.trim().to_string(); 
        
        if input.to_lowercase() == "exit" {
            println!("thanks for playing! goodbye!");
            break;
        }

        println!("\nwhat do u want me to do with your string?");
        println!("1. count characters");
        println!("2. reverse it");
        println!("3. make it uppercase");
        println!("4. make it lowercase");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read choice");

        let choice = choice.trim();

        if choice == "1" {
            let count = count_chars(&input);
            println!("ur string has {} characters.", count);
        } else if choice == "2" {
            let reversed = reverse_string(&input);
            println!("reversed: {}", reversed);
        } else if choice == "3" {
            let upper = to_uppercase(&input);
            println!("uppercase: {}", upper);
        } else if choice == "4" {
            let lower = to_lowercase(&input);
            println!("lowercase: {}", lower);
        } else {
            println!("invalid choice!");
        }
        
        println!("\npress enter to continue");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("failed to read input");
}

// ---- functions ----

// borrowed reference, does NOT take ownership
fn count_chars(s: &String) -> usize {
    s.len()
}

// borrowed reference, returns new String
fn reverse_string(s: &String) -> String {
    s.chars().rev().collect()
}

fn to_uppercase(s: &String) -> String {
    s.to_uppercase()
}

fn to_lowercase(s: &String) -> String {
    s.to_lowercase()
}

}