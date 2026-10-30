# Lesson 70: Organizing Code with Modules (`mod`, `pub`, `use`)

Congratulations on reaching **Lesson 70**! 🎉

As your programs grow from 50 lines to 50,000 lines, putting all your functions, structs, and logic into one single `main.rs` becomes an unmaintainable nightmare.

To organize large codebases, Rust provides a modular system:
* **Packages & Crates:** Cargo projects that build libraries or executables.
* **Modules (`mod`):** Containers that group related code and control privacy.
* **Visibility (`pub`):** Declaring what is public vs private.
* **Paths (`use`):** Navigating and importing items.

Let's learn how to organize Rust code like senior software architects! 🏗️

---

## 1. The Real-Life Analogy: The Restaurant Kitchen vs Dining Room 🍽️

Think of a sit-down restaurant:
* **The Front of House (Public `pub`):**
  * The dining tables, the menu, the host station.
  * Customers are welcome to walk in, read the menu, and order food.
* **The Back of House (Private by default):**
  * The kitchen, the spice cabinet, the chef's secret recipes.
  * Customers cannot walk into the kitchen and touch the stove!

In Rust:
> **Everything in Rust is PRIVATE by default!**  
> If you want an outside file or function to see an item, you must explicitly mark it with the **`pub` (public)** keyword!

---

## 2. Defining Modules with `mod`

You can define nested modules directly inside your file using the `mod` keyword:

```rust
// A module grouping kitchen operations:
mod kitchen {
    // Private by default: only functions inside `kitchen` can see this!
    fn wash_vegetables() {
        println!("Washing fresh carrots and lettuce...");
    }

    // Public: accessible to outside code!
    pub fn prepare_salad() {
        wash_vegetables(); // Kitchen can call its own private functions!
        println!("Tossing Mediterranean salad! 🥗");
    }
}

fn main() {
    // Call public function using double colon path:
    kitchen::prepare_salad();

    // kitchen::wash_vegetables();
    // ❌ COMPILER ERROR: function `wash_vegetables` is private!
}
```

### 📋 Expected Terminal Output:
```text
Washing fresh carrots and lettuce...
Tossing Mediterranean salad! 🥗
```

---

## 3. Bringing Paths into Scope with `use`

Typing `kitchen::orders::billing::calculate_tax()` every single time you call a function gets repetitive.  
The **`use`** keyword creates a convenient shortcut:

```rust
mod math_utils {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn square(x: i32) -> i32 { x * x }
}

// Create shortcuts:
use math_utils::{add, square};

fn main() {
    println!("Sum:    {}", add(10, 20));
    println!("Square: {}", square(5));
}
```

---

## 4. Splitting Code Across Multiple Files 📁

In real projects, each module gets its own dedicated file:

```text
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── network.rs    # Separate file for network code!
```

In `src/network.rs`:
```rust
pub fn connect() {
    println!("Connecting to secure satellite server...");
}
```

In `src/main.rs`:
```rust
// 1. Declare the module (Rust automatically finds network.rs!):
mod network;

// 2. Bring into scope:
use network::connect;

fn main() {
    connect();
}
```

> [!TIP]
> In Rust, you do **not** write `include "network.rs"` like C or C++.  
> Writing `mod network;` tells Rust: *"Go look for a file named `network.rs` in the same directory and compile it as a module!"*

---

## 5. Struct and Enum Visibility with `pub`

* **For Structs:** Marking a struct `pub` makes the struct visible, but its **fields remain private** unless each field is also marked `pub`!
  ```rust
  pub struct Breakfast {
      pub toast: String,      // Customer can choose
      seasonal_fruit: String, // Secret chef's choice (private)
  }
  ```
* **For Enums:** If you mark an enum as `pub`, **all of its variants automatically become public**!

---

## 6. Quick Recap Checklist
- [x] Modules group code and manage encapsulation boundaries using `mod`.
- [x] All items are **private by default** in Rust.
- [x] Use `pub` to make functions, structs, or modules public.
- [x] Use `use path::item;` to create local shortcuts.
- [x] Declare multi-file modules using `mod filename;`.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_70_modules_and_packages/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_70_modules_and_packages/TASK.md).
* You've completed Level 7! Next up: [Level 8: Advanced Mastery & Concurrency](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/71_generics_functions.md).
