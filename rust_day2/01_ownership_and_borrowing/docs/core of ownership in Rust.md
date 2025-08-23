

> **`String` lives on the heap (it owns a buffer plus `len` & `capacity`). `s` is the sole owner of that heap data. When `s` goes out of scope, the memory is freed.**

---

### 1. What does it mean that `String` lives on the heap?

In Rust:

* **Heap** = big area of memory where data can grow/shrink at runtime.
* **Stack** = smaller, fast memory for fixed-size values (like integers, pointers, function calls).

When you do:

```rust
let s = String::from("hello");
```

* On the **stack**, Rust stores the variable `s`.
* That variable `s` is not the whole string—it’s a **pointer to heap data** + two numbers: `len` and `capacity`.

So `s` looks like this in memory:

```
Stack:
s -> [ ptr | len | capacity ]

Heap:
ptr -> "hello"
```

* `ptr` → points to `"hello"` stored in the heap.
* `len` → how many characters are actually in use (`5` here).
* `capacity` → how much memory is reserved in advance (maybe `5`, maybe more).

---

### 2. What does it mean that `s` is the sole owner?

Rust has a strict rule: **each piece of heap memory must have exactly one owner.**

Here, `s` is the only variable that "owns" this `"hello"` buffer in the heap.

* If you try to assign `s` to another variable without explicitly cloning, ownership **moves** (we’ll see this later).
* No two variables can both think "this memory is mine"—that would risk double free (one frees it, then the other frees it again).

---

### 3. What happens when `s` goes out of scope?

At the end of the function/block, Rust automatically runs **drop** for `s`:

* It looks at the pointer stored in `s`.
* Frees the heap memory `"hello"`.
* Removes `s` itself from the stack.

So memory is **automatically cleaned up**, no garbage collector required.

---

In short:

* `String` = a wrapper (pointer, length, capacity) on the stack.
* Heap stores the actual text.
* `s` owns that text exclusively.
* When `s` is dropped, the heap memory is released safely.

---

Method call on a mutable reference. Desugars to `String::push_str(&mut *r3, "!")`

---


We’re looking at this:

```rust
r3.push_str("!");
```

### 1. What `r3` actually is

* Earlier, we declared something like:

  ```rust
  let mut s = String::from("hello");
  let r3 = &mut s;
  ```
* So `r3` is a **mutable reference (`&mut String`)** pointing to the same memory as `s`.
* That means `r3` doesn’t *own* the string — it just *borrows* it, but with permission to **modify**.

---

### 2. What happens when we call `.push_str("!")`

* Normally, if you had `s.push_str("!")`, it means:
  **“take this String `s` and add the characters `!` at the end.”**
* But `r3` is not a `String`, it’s a `&mut String`.
* Rust has **auto-dereferencing** (kind of automatic `*` under the hood).
* So `r3.push_str("!")` is equivalent to:

  ```rust
  (*r3).push_str("!");
  ```
* Which means: **dereference `r3` to access the underlying `String`, then call `.push_str("!")`.**

---

### 3. Why I wrote “desugars to `String::push_str(&mut *r3, "!")`”

* Every method call in Rust is “sugar” for a function call.
* For example:

  ```rust
  r3.push_str("!");
  ```

  is shorthand for:

  ```rust
  String::push_str(&mut *r3, "!");
  ```
* Here’s the breakdown:

  * `*r3` → gives us the actual `String` (not the reference).
  * `&mut *r3` → re-borrows it mutably, because the method `push_str` expects `&mut self`.
  * So effectively, the compiler rewrites it in that function call style.

---

### 4. The key idea: **mutation happens in place**

* Nothing is copied, nothing is cloned.
* The `String` `s` and the reference `r3` both point to the **same heap buffer**.
* After this line, `s` has actually changed — because `r3` was directly modifying its data.

---

So the meaning of

> “We’re mutating the original string via the reference; nothing is copied.”
> is:

* The string `"hello"` in memory becomes `"hello!"`.
* It wasn’t duplicated anywhere — only one copy exists, and `r3` just modified that copy.

---

