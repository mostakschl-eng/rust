# Lesson 74: Implementing Traits for Custom Types

In Lesson 73, we introduced the concept of Traits as capability contracts.  
Now, let's explore how traits enable true **polymorphism** in Rust: writing functions that work across multiple distinct types using **Trait Bounds**! 🤹‍♂️

---

## 1. The Real-Life Analogy: The Universal USB-C Port 🔌

Think of the USB-C standard:
* Your laptop has a USB-C port.
* The laptop doesn't care if you plug in:
  * A **Mechanical Keyboard**
  * A **4K Monitor**
  * A **Smart Phone**
  * An **External Hard Drive**
* As long as the device fulfills the **USB-C Electrical Specification (The Trait)**, the laptop can talk to it seamlessly!

---

## 2. Trait Bound Syntax: `<T: TraitName>`

In Lesson 73, we saw the short syntax `fn notify(item: &impl Summarizable)`.  
The longer, more explicit and powerful syntax is called a **Trait Bound**:

```rust
pub trait Area {
    fn area(&self) -> f64;
}

// 'T: Area' means: T can be ANY type, as long as T implements the Area trait!
fn print_area_info<T: Area>(shape: &T) {
    println!("Shape calculation: Area = {:.2} sq units", shape.area());
}
```

Let's implement `Area` for two completely different geometry structs:

```rust
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 10.0, height: 4.0 };

    print_area_info(&c); // Circles have an area!
    print_area_info(&r); // Rectangles have an area!
}
```

### 📋 Expected Terminal Output:
```text
Shape calculation: Area = 78.54 sq units
Shape calculation: Area = 40.00 sq units
```

---

## 3. Multiple Trait Bounds with the `+` Syntax

What if a function requires an item to have **two different capabilities** at the same time?  
For example, it must be `Summarizable` **AND** printable via `Display`?  
Combine them with the plus sign **`+`**:

```rust
use std::fmt::Display;

fn announce<T: Summarizable + Display>(item: &T) {
    println!("Announcement: {}", item);
    println!("Details:      {}", item.summarize());
}
```

---

## 4. Cleaning Up Complex Bounds with `where` Clauses

When you have multiple generic types with multiple bounds, function signatures can get crowded:
```rust
// ❌ Cluttered signature:
fn complex_process<T: Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) -> i32 { ... }
```

Rust gives you the **`where` clause** to clean this up:
```rust
// ✅ Crystal clear signature:
fn complex_process<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    // Clean function body!
    0
}
```

---

## 5. Quick Recap Checklist
- [x] Trait bounds constrain generic placeholders to types implementing specific traits: `fn run<T: Trait>(item: &T)`.
- [x] Combine multiple traits using the `+` operator: `<T: Trait1 + Trait2>`.
- [x] Use `where` clauses to organize complex trait bounds cleanly.
- [x] Trait bounds provide compile-time verified polymorphism with zero runtime overhead.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_74_implementing_traits/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_74_implementing_traits/TASK.md).
* Next lesson: [Lesson 75: Essential Standard Library Traits (`Display`, `Debug`, `Clone`)](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/75_standard_traits.md).
