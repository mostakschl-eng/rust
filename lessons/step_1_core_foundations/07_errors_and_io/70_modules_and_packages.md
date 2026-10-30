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
> **Module items are private by default.** They can be accessed by their defining module and its descendants. `pub` exposes an item through accessible module paths; files themselves do not define privacy boundaries. Public enum variants and trait items follow special visibility rules.

---

## 2. Defining Modules with `mod`

You can define nested modules directly inside your file using the `mod` keyword:

```rust
// A module grouping kitchen operations:
mod kitchen {
    // Private by default: accessible inside kitchen and its descendant modules
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
- [x] Module items are private by default, accessible within their module and descendants.
- [x] Use `pub` to make functions, structs, or modules public.
- [x] Use `use path::item;` to create local shortcuts.
- [x] Declare file-backed modules using `mod filename;`. A package has a Cargo manifest and may contain multiple crates; a crate is one compilation unit.

---


## Symbols and Practical Boundaries

A restaurant exposes its order counter while keeping recipes in the kitchen. Visibility is an API boundary, not cryptographic protection.

| Symbol or term | Meaning |
|---|---|
| `mod` | module |
| `pub` | public visibility |
| `use` | local name import |
| `::` | path separator |

**Common pitfall:** The toy hash_token only prefixes text and is not a password hash or secure authentication. Public Sensor fields are still part of its API, so changing them can break callers.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/07_errors_and_io/task_70_modules_and_packages/TASK.md`](../../../tasks/step_1_core_foundations/07_errors_and_io/task_70_modules_and_packages/TASK.md).
* You've completed Level 7! Next up: [Level 8: Advanced Mastery & Concurrency](../08_advanced_and_concurrency/71_generics_functions.md).
