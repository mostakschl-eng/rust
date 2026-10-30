# Lesson 45: Associated Functions & Constructors (`Self::new`)

In Lesson 44, our methods had `&self` as their first parameter, meaning they needed an **already-existing instance** to be called:
```rust
rect.area(); // Called on rect!
```

What if you want to write a function that **creates the instance in the first place**?  
Or a utility function related to the type that doesn't need an existing object?

In Rust, these are called **Associated Functions**! 🏭

---

## 1. The Real-Life Analogy: The Car Factory Assembly Line 🏭

Think about buying a new car:
* You don't ask an existing car driving on the street: *"Hey car, please manufacture a brand-new baby car for me!"*
* That makes no sense. An existing car doesn't build new cars.
* Instead, you go to the **Car Factory**:
  * You call the factory blueprint: `Car::manufacture("Red", "Sedan")`.
  * The factory constructs and hands you a brand-new car!

In Rust:
> An **associated function** belongs to a type (or trait). A function with a `self` receiver is also an associated function, specifically a **method**. Here we compare methods with associated functions that do **not** take `self`.  
> Because it has no `self`, it is associated with the **Type itself**, not a specific instance!

---

## 2. The Universal Constructor Convention: `Self::new`

Rust does not have a special `new` keyword (like `new Car()` in Java, C++, or JavaScript).  
Instead, the standard Rust convention is to write an associated function named **`new`**:

```rust
#[derive(Debug)]
struct Player {
    username: String,
    score: u32,
    lives: u8,
}

impl Player {
    // Associated constructor function:
    // 'Self' (with a capital S) is an alias for the struct type (Player)
    fn new(username: String) -> Self {
        Self {
            username,
            score: 0,
            lives: 3,
        }
    }

    // Regular method (takes &self):
    fn display_status(&self) {
        println!("Player: {} | Score: {} | Lives: {}", self.username, self.score, self.lives);
    }
}

fn main() {
    // Call associated function using double colons (::):
    let player1 = Player::new(String::from("DragonSlayer"));

    // Call method using dot (.):
    player1.display_status();
}
```

### 📋 Expected Terminal Output:
```text
Player: DragonSlayer | Score: 0 | Lives: 3
```

Notice the difference in punctuation:
* `Type::function()` -> Associated function (Factory/Constructor)
* `instance.method()` -> Method (Action on an existing object)

---

## 3. Alternative Constructors: Named Factories

You aren't limited to just `new`! You can create multiple constructors for different scenarios:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Standard constructor:
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Square factory: creates a rectangle where width == height!
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(25);

    println!("Rectangle: {:?}", rect);
    println!("Square:    {:?}", sq);
}
```

### 📋 Expected Terminal Output:
```text
Rectangle: Rectangle { width: 30, height: 50 }
Square:    Rectangle { width: 25, height: 25 }
```

---

## 4. `String::from` and `Vec::new` Unveiled! 💡

Remember back in Lesson 35 and 38 when we wrote:
```rust
let s = String::from("hello");
let v: Vec<i32> = Vec::new();
```
Now you know what they actually are!
* `String::from` is an **associated function** on the `String` type!
* `Vec::new` is an **associated function** on the `Vec` type!
* The double colon `::` is Rust's way of navigating namespaces and associated items.

---

## 5. Summary: Methods vs Associated Functions

| Feature | Method | Associated Function |
|---|---|---|
| **Receiver** | `&self`, `&mut self`, or `self` | No `self` receiver; ordinary parameters are allowed |
| **How to Call** | `instance.method()` | `Type::function()` |
| **Needs an Object?** | Yes, runs on existing instance | No, runs on the type itself |
| **Typical Purpose** | Reading or modifying state | Creating new objects (`new`, `from`, `default`) |

---

## 6. Quick Recap Checklist
- [x] Associated functions live inside `impl StructName` but do not take `self`.
- [x] Called using the double colon `Type::function_name()`.
- [x] The `Self` keyword inside an `impl` block refers to the struct type.
- [x] `new` is the conventional name for constructors in Rust.

---

## Pitfall: `new` has no built-in magic

`new` is an ordinary name chosen by convention; you write its body and validation yourself. Associated functions may have several parameters. `Self` names the type inside its implementation; lowercase `self` is the current receiver. Methods can also be called with type syntax, for example `Player::display_status(&player1)`.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `Self` | The type being implemented |
| `self` | A specific receiver value |
| `Server::new(...)` | Call a function without a receiver |
| `server.url()` | Call a method on a receiver |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_45_associated_functions/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_45_associated_functions/TASK.md).
* Next lesson: [Lesson 46: Enums: Representing Fixed Choices](46_enums_basics.md).
