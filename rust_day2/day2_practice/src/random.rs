// Rules of Borrowing (VERY IMPORTANT ⚡)
// Rust enforces strict borrowing rules to guarantee memory safety:
// You can have any number of immutable references (&T) OR exactly one mutable reference (&mut T) to a value at the same time.
// ✅ Many readers allowed.
// ✅ One writer allowed.
// ❌ Readers + Writer at the same time (conflict).
// References must always be valid (they cannot outlive the value they point to).
// Example (compiler errors explained):
// ❌ Multiple mutable borrows:

let mut s = String::from("hi");
let r1 = &mut s;
let r2 = &mut s; // ERROR: cannot borrow mutably more than once


//❌ Immutable + mutable borrow together:

let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s; // ERROR: cannot borrow `s` as mutable because it's already borrowed as immutable


// Why These Rules Exist (The “No Data Races” Guarantee)
// Immutable refs: Safe to read from multiple places at once → no problem.
// Mutable ref: Needs exclusive access → avoids race conditions, dangling pointers, unexpected changes.
// Compiler enforces this at compile-time, so you never get runtime crashes like in C/C++.
