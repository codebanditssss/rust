// A struct (short for structure) is a way to group related data together.
// Think of it like a custom data type that you create yourself, 
// similar to classes in other languages but simpler (no inheritance).

struct Rectangle {
    width: u32,
    height: u32, //ERROR (missing comma)
}



// Rectangle is the name of the struct.
// It has two fields: width and height, both of type u32.
// Each instance of Rectangle will have its own values for these fields.    

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("the rectangle has width {} and height {}", rect1.width, rect1.height);
}
