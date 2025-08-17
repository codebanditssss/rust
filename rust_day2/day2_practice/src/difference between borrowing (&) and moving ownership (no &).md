The difference between **borrowing (`&`)** and **moving ownership (no `&`)**.

---

### Example 1: With `&` (Borrowing)

```rust
fn main() {
    let s = String::from("hello");

    // Borrow `s`, not move it
    print_string(&s);

    // Still can use s
    println!("In main: {}", s);
}

fn print_string(s: &String) {
    println!("In function: {}", s);
}
```

---

### Word by Word Explanation

#### `let s = String::from("hello");`

* `let` → keyword to create a variable.
* `s` → name of the variable.
* `String::from("hello")` → creates a new heap-allocated string with the content `"hello"`.
* So, now ownership of `"hello"` is with `s`.

---

#### `print_string(&s);`

* `print_string` → function we defined below.
* `(&s)` → `&` means *borrow a reference* to `s` instead of moving ownership.
* So the function only *looks at* `s`, it doesn’t take it away.
* After this line, `s` is **still owned by main**.

---

#### `println!("In main: {}", s);`

* We can still use `s` because ownership never moved, only a reference was borrowed.
* Output: `In main: hello`

---

#### `fn print_string(s: &String) { ... }`

* `fn` → defines a function.
* `print_string` → function name.
* `(s: &String)` → parameter named `s`, but note `&String`:

  * means “I only borrow a reference to a String”
  * The function cannot take ownership.

Inside:
`println!("In function: {}", s);` → just prints the borrowed string.

---

### Example 2: Without `&` (Move Ownership)

```rust
fn main() {
    let s = String::from("hello");

    // Move ownership to the function
    print_string(s);

    // ❌ ERROR: s is no longer valid here
    println!("In main: {}", s);
}

fn print_string(s: String) {
    println!("In function: {}", s);
}
```

---

### Word by Word Here

#### `print_string(s);`

* Here, we pass `s` **directly**, not `&s`.
* That means ownership of `"hello"` is *moved* from `main` into the function.

---

#### Inside function

* The function parameter is `s: String`, so now `print_string` owns the string.
* It prints it.
* At the end of the function, the string is dropped (freed).

---

#### Back in `main`

* When we try `println!("In main: {}", s);` → compiler error!
* Because `s` has been **moved**, it’s no longer valid.

---

✅ **Key takeaway**:

* `&s` → Borrow → don’t lose ownership → can still use after function call.
* `s` → Move → lose ownership → can’t use after function call.

---

