// ownership rules 


// 1. each value in rust has a single owner
// let s = String::from("hello"); // s owns the string

// 2. when the owner goes out of scope, the value is dropped
// {
//     let s = String::from("world");
// } // s goes out of scope here, memory freed

// 3. when ownership is moved, the old owner can’t use it anymore
// let s1 = String::from("hi");
// let s2 = s1; // ownership moves to s2
// // println!("{}", s1); // error: s1 no longer valid
