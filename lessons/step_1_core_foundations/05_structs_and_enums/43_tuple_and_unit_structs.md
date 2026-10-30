# Lesson 43: Tuple Structs & Unit-Like Structs

In Lessons 41 and 42, our structs had **named fields** like `username`, `email`, and `port`.  
Rust also provides two specialized, lightweight struct variations:
1. **Tuple Structs** (Structs with unnamed, numbered fields).
2. **Unit-Like Structs** (Structs with zero fields at all!).

Let's understand why they exist and how they make your code safer! 🎨

---

## 1. Tuple Structs: The Real-Life Analogy

### The Mix-Up Problem:
Imagine a 3D graphic program where you represent:
* A 3D Spatial Coordinate: `(x, y, z)` e.g. `(255, 0, 128)`
* An RGB Screen Color: `(red, green, blue)` e.g. `(255, 0, 128)`

Both are just three integers!  
If you used plain tuples `(i32, i32, i32)`, you could accidentally pass a **Color** into a function that moves the camera **Position**! The computer wouldn't catch the mistake!

In Rust:
> A **Tuple Struct** gives a distinct name and unique type to a tuple.  
> `Color` and `Point` are treated as **completely different types** by the compiler, even though both hold three integers!

---

## 2. Defining and Using Tuple Structs

Tuple structs use parentheses `( )` instead of curly braces `{ }`, followed by a semicolon `;`:

```rust
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point3D(i32, i32, i32);

fn apply_screen_color(c: Color) {
    println!("Painting screen with RGB: ({}, {}, {})", c.0, c.1, c.2);
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point3D(0, 0, 0);

    apply_screen_color(black); // ✅ Valid!

    // apply_screen_color(origin);
    // ❌ COMPILER ERROR: expected `Color`, found `Point3D`!
}
```

### Accessing Tuple Struct Fields:
Just like regular tuples, you access fields using dot and index:
```rust
let sky = Color(135, 206, 235);
println!("Red channel:   {}", sky.0);
println!("Green channel: {}", sky.1);
println!("Blue channel:  {}", sky.2);
```

You can also destructure them:
```rust
let Color(r, g, b) = sky;
println!("Unpacked Red: {}", r);
```

---

## 3. The "Newtype Pattern" (Domain Modeling)

A common use of tuple structs with a single field is called the **Newtype Pattern**:
```rust
struct Meters(f64);
struct Feet(f64);

fn calculate_orbit(distance: Meters) {
    println!("Calculating orbital velocity for {} meters...", distance.0);
}

fn main() {
    let runway_length = Meters(3500.0);
    let mountain_height = Feet(14000.0);

    calculate_orbit(runway_length); // ✅ Clean!
    // calculate_orbit(mountain_height); // ❌ Caught at compile time!
}
```
Distinct types can catch unit mix-ups at compile time when functions require them; correct unit conversions still need to be implemented.

---

## 4. Unit-Like Structs: Zero-Sized Types! 🏷️

What if you want to define a struct that holds **no data at all**?  
This is called a **Unit-Like Struct** (because it behaves like the empty unit tuple `()`):

```rust
struct AlwaysTrueValidator;

fn main() {
    let validator = AlwaysTrueValidator;
    println!("Validator initialized with 0 bytes of memory!");
}
```

### Why would you ever want a struct with no data?
Unit-like structs are used when you need to implement a **behavior (Trait)**, but you don't need to store any state! Traits are covered later; for now, a unit-like struct gives a name to a value with no fields.

---

## 5. Summary Comparison

| Struct Type | Syntax | Best Used For |
|---|---|---|
| **Named Struct** | `struct User { name: String }` | Rich domain entities with self-documenting fields |
| **Tuple Struct** | `struct Point(i32, i32);` | Grouped values where field names are obvious or Newtype wrappers |
| **Unit-Like Struct** | `struct Marker;` | Trait implementations and zero-cost markers |

---

## 6. Quick Recap Checklist
- [x] Tuple structs look like `struct Name(Type1, Type2);` with a trailing semicolon.
- [x] Fields are accessed via `.0`, `.1`, or destructuring.
- [x] Two tuple structs with identical field types are distinct, incompatible types (Type Safety!).
- [x] Unit-like structs hold no data and consume zero bytes of memory.

---

## Expected output of the first complete example

```text
Painting screen with RGB: (0, 0, 0)
```

## Pitfall: a type name is not automatic validation

`Meters(f64)` keeps meters separate from feet, but it still accepts negative or non-finite numbers unless you add rules. Likewise, constructing a marker named `SecurityKey` does not authenticate anybody. Its value has size zero; references and containers holding it can still take space.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `struct Color(u8, u8, u8);` | Define a tuple struct |
| `Color(...)` | Construct that named type |
| `let Point3D(x, y, z)` | Destructure its fields |
| `struct SecurityKey;` | Define a fieldless marker |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_43_tuple_and_unit_structs/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_43_tuple_and_unit_structs/TASK.md).
* Next lesson: [Lesson 44: Methods: Attaching Functions to Structs (`impl`, `&self`)](44_struct_methods.md).
