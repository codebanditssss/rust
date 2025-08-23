# advanced enum patterns explained simply

these are the advanced enum concepts i just learned. some are complex for day 2, but here's my breakdown.

## patterns i learned

### 1. range patterns

```rust
match number {
    1..=10 => println!("small"),
    11..=50 => println!("medium"), 
    _ => println!("large"),
}
```

how it works:
- instead of writing `1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10`, i can write `1..=10`
- the `..=` means "from 1 to 10, including both 1 and 10"
- works with numbers and letters like `'a'..='z'`
- much cleaner than listing every possibility

when i'd use this:
- grouping ranges of values
- categorizing things like grades (a-f), age groups, etc.

### 2. multiple patterns with |

```rust
match day_number {
    1 | 2 | 3 | 4 | 5 => println!("weekday"),
    6 | 7 => println!("weekend"),
    _ => println!("invalid"),
}
```

how it works:
- the `|` means "or"
- so `1 | 2 | 3` means "if it's 1 or 2 or 3"
- groups related values together

when i'd use this:
- when several different values should be handled the same way
- cleaner than writing separate match arms for each

### 3. guard clauses with if

```rust
match num {
    x if x < 10 => println!("small: {}", x),
    x if x % 2 == 0 => println!("even: {}", x),
    x => println!("other: {}", x),
}
```

how it works:
- after matching a pattern, i can add extra conditions with `if`
- `x if x < 10` means "match any number, call it x, but only if x is less than 10"
- the `if` part is checked after the pattern matches

when i'd use this:
- when the pattern alone isn't enough to decide what to do
- for complex conditions that can't be expressed in simple patterns

mental model:
- pattern matching says "what shape is this?"
- guard clause says "ok it's that shape, but does it meet this extra requirement?"

### 4. destructuring structs

```rust
struct Point { x: i32, y: i32 }

match point {
    Point { x: 0, y: 0 } => println!("origin"),
    Point { x: 0, y } => println!("on y-axis at {}", y),
    Point { x, y: 0 } => println!("on x-axis at {}", x),
    Point { x, y } => println!("point at ({}, {})", x, y),
}
```

how it works:
- i can pull apart structs in the match
- `Point { x: 0, y }` means "match points where x is exactly 0, and grab the y value"
- `Point { x, y }` means "grab both x and y values"

when i'd use this:
- extract data from structs while also checking their values
- handle special cases (like origin point) differently

### 5. ignoring parts with _

```rust
let tuple = (1, 2, 3, 4, 5);
match tuple {
    (first, _, _, _, last) => {
        println!("first: {}, last: {}", first, last);
    }
}
```

how it works:
- `_` means "i don't care about this value"
- useful when i only need some parts of a larger structure

### 6. @ bindings

```rust
match value {
    Some(x @ 40..=50) => {
        println!("found number in range: {}", x);
    },
    _ => {},
}
```

how it works:
- `x @ 40..=50` means "check if it's in range 40-50, and if so, call it x"
- i get both the test (is it in range?) and the value (what is x?)
- normally i'd need two steps: test the range, then extract the value

mental model: @ means "capture this while also testing it"

when i'd use this:
- when i need both to test a pattern and use the matched value
- avoids having to extract the value in a separate step

### 7. reference patterns

```rust
let numbers = vec![1, 2, 3];
for number in &numbers {
    match number {
        1 => println!("one"),
        x if *x > 2 => println!("big: {}", x),
        _ => println!("other"),
    }
}
```

how it works:
- when iterating with `&numbers`, each `number` is a reference
- sometimes rust automatically handles the reference for me
- sometimes i need `*x` to get the actual value from the reference

### enum methods

```rust
impl TrafficLight {
    fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
}
```

how it works:
- i can add methods to enums just like structs
- `&self` means the method looks at the current enum value
- inside the method, use `match` to do different things for different variants

why this is useful:
- instead of having separate functions that take enums
- i can call methods directly on the enum: `light.wait_time()`
- keeps related behavior close to the data

### associated functions (constructors)

```rust
impl TrafficLight {
    fn new_red() -> TrafficLight {
        TrafficLight::Red
    }
}
```

how it works:
- functions that belong to the enum type but don't take `&self`
- usually used to create new instances
- call them like `TrafficLight::new_red()`

## complexity factors

these patterns combine multiple concepts:
- pattern matching + conditions + destructuring
- ownership and borrowing rules
- heavy syntax with many symbols

## usage levels

**basic patterns:**
- simple match with enum variants
- option and result handling
- basic enum methods

**intermediate patterns:**
- range patterns: `1..=10`
- multiple patterns: `1 | 2 | 3`
- guard clauses: `x if x > 10`
- struct destructuring: `Point { x, y }`

**advanced patterns:**
- @ bindings: `x @ 40..=50`
- nested destructuring
- reference patterns with `&` and `*`

## core concepts needed

- enums store different types of data
- match handles all cases exhaustively
- option prevents null pointer errors
- if let simplifies single-pattern matches
