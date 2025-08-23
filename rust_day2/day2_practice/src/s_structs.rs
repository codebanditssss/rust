// What is Shadowing in Structs?

// In Rust, shadowing happens when a variable name inside a method or function has the same name as a struct field.
// This can lead to confusion, because the local variable will take precedence, making the struct field inaccessible unless you use self. explicitly.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        // Here, `width` (parameter) shadows `self.width` (struct field)
        self.width = width; 
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };
    println!("Before: width = {}, height = {}", rect.width, rect.height);

    rect.set_width(50);
    println!("After: width = {}, height = {}", rect.width, rect.height);
}

// In the above code:

// The parameter width shadows the struct’s field width.
// To refer to the struct’s field, you must use self.width.
// Without self., Rust won’t know you mean the field, not the parameter.

// common Mistakes

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        width = width; //  Wrong! This just assigns the parameter to itself.
    }
}

// This does nothing and won’t update the struct.
// You need self.width = width;.


// Alternative Style (Avoiding Shadowing)
// You can use different parameter names to avoid shadowing:

impl Rectangle {
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;  // Clearer, no shadowing confusion
    }
}

// Key Takeaway

// Shadowing is allowed, but can lead to mistakes.
// Best practice: either use self.field explicitly OR use different parameter names.