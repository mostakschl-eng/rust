# Lesson 31: Grouping Fixed Data: Tuples

Welcome to **Level 4: Compound Types & Collections**! 🎒

Up until now, our variables held only **one single value** at a time:
```rust
let name = "Alice";
let age = 30;
let is_active = true;
```

What if we want to bundle Alice's name, age, and status together into **one single package**?  
That is what **Tuples** are for!

---

## 1. The Real-Life Analogy: The Bento Box 🍱

Imagine you order a Japanese Bento Lunch Box:
* Slot 1 holds: **Chicken Teriyaki** (Meat)
* Slot 2 holds: **Steamed Rice** (Carb)
* Slot 3 holds: **Pickled Ginger** (Vegetable)

Each slot holds a **different kind of food**, but they are all delivered inside **one single container**!

In Rust:
> A **Tuple** is a fixed-size container that bundles multiple values together.  
> Unlike arrays, the values inside a tuple can have **different data types**!

---

## 2. Your First Tuple in Rust

To create a tuple, wrap comma-separated values inside round parentheses `( )`:

```rust
fn main() {
    // A tuple holding text, integer, and decimal:
    let employee = ("Alice Johnson", 32, 85000.50, true);

    // Access elements using dot (.) followed by the index (starting at 0):
    println!("Name:       {}", employee.0);
    println!("Age:        {}", employee.1);
    println!("Salary:     ${}", employee.2);
    println!("Active:     {}", employee.3);
}
```

### 📋 Expected Terminal Output:
```text
Name:       Alice Johnson
Age:        32
Salary:     $85000.5
Active:     true
```

---

## 3. How to Read Tuple Syntax

| Syntax | Meaning | Example |
|---|---|---|
| `(val1, val2, ...)` | Creates a tuple | `("Apple", 5, 2.50)` |
| `tuple.0` | Accesses the 1st element | `employee.0` |
| `tuple.1` | Accesses the 2nd element | `employee.1` |
| `tuple.2` | Accesses the 3rd element | `employee.2` |
| `(&str, i32, f64)` | Explicit Type Signature | `let item: (&str, i32, f64) = ("Pen", 10, 1.25);` |
| `()` | Empty tuple ("Unit Type") | Returned by functions that don't return anything! |

> [!NOTE]
> Rust tuple fields and array indices start at **0**.  
> The 1st item is at index `.0`, the 2nd item is at `.1`, the 3rd is at `.2`, and so on.

---

## 4. Mutable Tuples: Modifying Elements

If you declare a tuple with `mut`, you can change the values inside—**as long as you keep the same data type**!

```rust
fn main() {
    let mut coordinates = (10, 20);
    println!("Old point: ({}, {})", coordinates.0, coordinates.1);

    // Update coordinates:
    coordinates.0 = 55;
    coordinates.1 = 99;
    println!("New point: ({}, {})", coordinates.0, coordinates.1);
}
```

> [!WARNING]
> You **cannot** change the type of a tuple slot after declaring it!  
> If `coordinates.0` is an integer, you cannot assign `"Hello"` to it.

---

## 5. Printing an Entire Tuple with `{:?}`

You cannot print an entire tuple directly with `{}` because Rust doesn't know what format you want.  
Instead, use the **Debug Placeholder** `{:?}`:

```rust
fn main() {
    let point = (12, 45, -3);
    println!("The 3D point is: {:?}", point);
}
```
**Output:**
```text
The 3D point is: (12, 45, -3)
```

---

## 6. Common Beginner Traps ⚠️

### Trap 1: A One-Element Tuple Needs a Trailing Comma!
If you write `(5)`, Rust thinks you wrote the number 5 inside math parentheses.  
To make a single-element tuple, add a comma:
```rust
let not_a_tuple = (5);    // Type: i32
let is_a_tuple  = (5,);   // Type: (i32,)
```

### Trap 2: Tuples Cannot Grow or Shrink!
Once you create a tuple with 3 items, it has **exactly 3 items forever**.  
You cannot add a 4th item. (For lists that can grow, we use **Vectors** in Lesson 35!)

---

## 7. Quick Recap Checklist
- [x] Tuples bundle multiple values of different types into one container `(a, b, c)`.
- [x] Access values using dot notation: `data.0`, `data.1`.
- [x] Counting starts at index `0`.
- [x] Print full tuples using the debug placeholder `{:?}`.
- [x] The size and types of a tuple are fixed at creation time.

---

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `(a, b)` | Bundle values into a tuple |
| `parcel.0` | Read the first field |
| `{:?}` | Debug formatting |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_31_tuples/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_31_tuples/TASK.md).
* Next lesson: [Lesson 32: Unpacking Tuples (Destructuring)](32_tuple_destructuring.md).
