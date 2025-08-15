
### **1️ `.parse()`**

* `.parse()` is a **method** in Rust (a function attached to a type) that tries to **convert a string (`&str`) into another type**, like `i32`, `f32`, etc.
* Its signature roughly looks like:

```rust
fn parse<T: std::str::FromStr>(&self) -> Result<T, T::Err>
```

* Notice it returns a **Result type** — meaning the conversion **might fail**.

  * `Ok(value)` → conversion succeeded
  * `Err(error)` → conversion failed (e.g., `"abc"` cannot be parsed to an integer)

Example:

```rust
let s = "42";
let num: Result<i32, _> = s.parse(); 
// num is Result<i32, ParseIntError>
```

---

### **2️ `.unwrap()`**

* `.unwrap()` is a **method on a `Result` type**.
* It does one thing:

  * If the `Result` is `Ok(value)`, it **extracts the value**.
  * If the `Result` is `Err(error)`, it **panics** (program crashes with an error message).

Example:

```rust
let s = "42";
let num: i32 = s.parse().unwrap(); // Ok(42) → extracts 42
```

* If `s = "abc"`:

```rust
let s = "abc";
let num: i32 = s.parse().unwrap(); 
// panic: "called `Result::unwrap()` on an `Err` value"
```

---

### **3️ How They Work Together**

```rust
let x = "5";          // &str
let x: i32 = x.parse().unwrap(); // convert string → i32
```

* `.parse()` → tries conversion → Result\<i32, \_>
* `.unwrap()` → extracts the value if conversion succeeds
* Now `x` is **type i32**, not \&str

---

 **Rule of thumb:**

* `.parse()` = try to convert string to number
* `.unwrap()` = force the conversion to succeed, or crash if it fails


