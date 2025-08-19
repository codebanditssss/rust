This is about **structuring your Rust code into multiple files and folders** instead of keeping everything in `main.rs`.

---

## Goal of this part:

* Show **how to organize code** using **modules** (`mod`)
* Explain **visibility** (`pub`)
* Demonstrate calling functions across files

---

## Step-by-step Breakdown

### 1. Why Modules?

* In small programs, everything can go into `main.rs`.
* But as the project grows, it becomes messy.
* **Modules** let us split code into **smaller, reusable files**.

Analogy:
Think of modules like **chapters in a book** → each chapter focuses on one theme, but together they make the whole book.

---

### 2. Simple Module inside `main.rs`

```rust
fn main() {
    // Calling a function from our utils module
    utils::greet();
}

// Define a module named utils
mod utils {
    pub fn greet() {
        println!("Hello from utils module!");
    }
}
```

Notes:

* `mod utils` → creates a module.
* Inside, we define `greet`.
* `pub` → makes it **public**, otherwise main() can’t access it.

---

### 3. Moving Module to a File 

Project structure:

```
src/
 ├── main.rs
 └── utils.rs
```

**main.rs**

```rust
mod utils; // bring utils.rs

fn main() {
    utils::greet();
}
```

**utils.rs**

```rust
pub fn greet() {
    println!("Hello from utils.rs!");
}
```

Rust automatically looks for a file with the same name as the module.

---

### 4. Nested Modules 

```
src/
 ├── main.rs
 └── utils/
     ├── mod.rs
     └── math.rs
```

**main.rs**

```rust
mod utils;

fn main() {
    utils::greet();
    utils::math::add(5, 10);
}
```

**utils/mod.rs**

```rust
pub fn greet() {
    println!("Hello from utils!");
}

pub mod math;
```

**utils/math.rs**

```rust
pub fn add(a: i32, b: i32) {
    println!("Sum = {}", a + b);
}
```

---




