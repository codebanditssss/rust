// A struct (short for structure) is a way to group related data together.
// Think of it like a custom data type that you create yourself, 
// similar to classes in other languages but simpler (no inheritance).

// struct Rectangle {
//     width: u32,
//     height: u32, //ERROR (missing comma)
// }



// Rectangle is the name of the struct.
// It has two fields: width and height, both of type u32.
// Each instance of Rectangle will have its own values for these fields.    

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!("the rectangle has width {} and height {}", rect1.width, rect1.height);
// }


// tuple structs in rust 

// a tuple struct looks like a tuple but with a name
// unlike a regular tuple(u32,u32), a tuple struct gives a semantic label to that grouping
// unlike a named filed struct its fields dont have names only positions(index based)


// can say it as hybrid between a tuple and a struct 


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// here color and point are diff types even though both hold three i32 values
// field are acceseed using a dot with index

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("black order rgb: {}, {}, {}", black.0, black.1, black.2);
    println!("origin point: {}, {}, {}", origin.0, origin.1, origin.2);
}


// diff between struct and tuple struct 

// named struct: fields have labels, bettter readability
// tuple struct: fields accessed by position, can be useful when labels arent needed

// struct Rectangle { widht: u32, height: u32 }  // descriptive 
// struct Dimensions(u32, u32); //compact

// mistake to avoid 

// let black = Color{ 0, 0, 0}; // wrong...needs parentheses not braces

// correct 
// let black = Color(0, 0, 0); // correct...uses parentheses