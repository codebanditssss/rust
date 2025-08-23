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

//----------------------------------------------------------------


// borrowing 

// instead of taking ownership, you can borrow a value with &
// multiple immutable borrows are allowed at the same time
// only one mutable borrow is allowed at a time (exclusive access)

// references (&T and &mut T)

// rust uses references to implement borrowing
// &T -> immutable reference (read-only access)
// &mut T → mutable reference (read and write access)


fn main() {
    let s = String::from("hello");
    //string lives on the heap (it owns a buffer plus len & capacity)
    //s is the sole owner of that heap data. when s goes out of scope, the memory is freed

    // Borrow immutably
    let r1 = &s;
    let r2 = &s;

    // &s creates immutable references to s’s data
    // no copy of the string data is made; r1/r2 are just small pointers
    // rust allows any number of immutable borrows at the same time (many readers, zero writers)
    // references like &String are copy (copying the reference is cheap and doesn’t move the string)

    println!("r1 = {}, r2 = {}", r1, r2);

    // if u later needed a mutable borrow of s, it would now be allowed because the immutable borrows are no longer in use

    // Borrow mutably
    let mut s2 = String::from("world");  // new owned string on the heap
    // the binding is mut, meaning the variable s2 can be changed (eg, its contents modified or reassigned)
    // rule: at any given time either any number of &T or exactly one &mut T
    // while r3 exists, no other borrows of s2 (mutable or immutable) are allowed
    let r3 = &mut s2;
    r3.push_str("!");
    // method call on a mutable reference. desugars to String::push_str(&mut *r3, "!")
    // we r mutating the original string via the reference; nothing is copied
    println!("r3 = {}", r3);
    // prints the contents via the mutable reference
    // after this last use of r3, the mutable borrow ends; s2 is free to be used again if there were more code
}

// many readers or one writer, never both at once -> prevents data races
// references can’t outlive owners (compiler enforces lifetimes)
// no implicit cloning of heap data when borrowing -> borrows are zero-cost pointers
// non-lexical lifetimes: a borrow ends when it’s last used, not necessarily at the end of the block, which makes code like this ergonomic

// things that would error here (and why)

// creating two mutable borrows of s2 at the same time
// let r4 = &mut s2; let r5 = &mut s2; → X (two writers)

// using s2 immutably while r3 (a mutable borrow) is still in use
// let r3 = &mut s2; println!("{}", s2); before last use of r3 → X (reader + writer overlap)

// trying to mutate through an immutable reference
// let r = &s2; r.push_str("!"); → X (needs &mut)


// Mental model to remember
// Library rule: Many people can read a book at once.
// Only one person can write in it at a time, and if someone’s writing, no one else can read simultaneously.
// Owner = librarian (controls the book’s lifetime). References = borrowers.