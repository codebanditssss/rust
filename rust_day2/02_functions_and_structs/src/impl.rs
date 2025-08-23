// adding methods with impl

// impl is a block where u define functions associated with a struct
// inside impl, functions that take &self(or &mut self, or self) become methods that can be called with dot notation

// methods let u model object like behavior in rust

//syntax

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {width :10, height: 5};
    println!("the area is {}", rect.area());
}

// impl Rectangle { ... }
// The impl block is where you implement functionality for your struct.
// Any functions you define inside are called methods if they take self (or &self / &mut self) as a parameter.
// Without impl, a struct is just data storage. With impl, it becomes a data + behavior unit (object-like).

// fn area(&self)
// this is a method attached to Rectangle

// &self shorthand
// &self is short for 

//fn area(self: &Rectangle) -> u32

// meaning this method borrows the rectangle instead of taking ownership
// so u can still use rect after calling .area()


// if it were just self: that would mean fn area(self) -> u32
// in that case calling .area() consumes the rectangle(moves ownership)
// not ideal bcuz u usually want to calculate area multiple times without losing the rectangle


// why borrowing (&self) is important

impl Rectangle {
    fn consume(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    println!("{}", rect.consume()); 
    // println!("{}", rect.consume()); // error: rect was moved
}


// here, the first call moves rect. after that, you can’t use it again
// with &self, you can safely reuse the same instance

// explicit return type

// rust requires explicit return types for functions
// so this works:

fn area(&self) -> u32 {
    self.width * self.height
}


// but this does not:

fn area(&self) {
    self.width * self.height // expected `()`, found `u32`
}


// because by default, functions without -> type return the unit type ()




// impl makes your struct powerful.
// &self ensures you don’t accidentally consume your struct.
// Always specify the return type clearly.