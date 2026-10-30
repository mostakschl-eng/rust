# Lesson 32: Unpacking Tuples (Tuple Destructuring)

In the previous lesson, we accessed tuple elements using dots: `box.0`, `box.1`, `box.2`.  
While that works, writing `.0`, `.1`, `.2` all over your code can feel awkward and impersonal:
* What is `data.0`? Is it an age? A price? An ID?
* What is `data.1`?

Rust has a cleaner, more readable superpower for this called **Destructuring** (Unpacking)! 🎁

---

## 1. The Real-Life Analogy: Unpacking a Delivery Box 📦

Imagine a delivery courier drops off a sealed cardboard box at your front door:
* Inside the box are 3 items: **A Book, A Coffee Mug, and A Notebook**.
* You don't leave them in the box and refer to them as "Box Item 0" and "Box Item 1".
* Instead, you **open the box** and place each item into its own place:
  * The book goes onto your **bookshelf**.
  * The mug goes into your **kitchen cabinet**.
  * The notebook goes onto your **desk**.

In Rust:
> **Destructuring** means taking all the individual pieces out of a tuple and binding each piece to its own named variable in one clean line!

---

## 2. Destructuring in Action

Notice how the left side mirrors the shape of the tuple on the right:

```rust
fn main() {
    // 1. Pack the tuple:
    let player_stats = ("ShadowNinja", 95, 3450.75);

    // 2. Unpack (destructure) into three distinct variables:
    let (username, health, score) = player_stats;

    // 3. Now use the variables directly with clear, readable names!
    println!("=== PLAYER PROFILE ===");
    println!("Username: {}", username);
    println!("Health:   {}%", health);
    println!("Score:    {} pts", score);
}
```

### 📋 Expected Terminal Output:
```text
=== PLAYER PROFILE ===
Username: ShadowNinja
Health:   95%
Score:    3450.75 pts
```

Look how clean and readable `username`, `health`, and `score` are compared to `player_stats.0`!

---

## 3. Ignoring Values with the Underscore `_`

What if a tuple has 4 items, but you only care about 2 of them?  
Rust lets you use the **underscore `_`** as a wildcard throwaway:

```rust
fn main() {
    let order = ("Laptop", 1, 1200.00, "Express Shipping");

    // We only care about the item name and the total price:
    let (item_name, _, price, _) = order;

    println!("Item: {} | Total: ${}", item_name, price);
}
```
**Output:**
```text
Item: Laptop | Total: $1200
```
> [!TIP]
> Using `_` tells the Rust compiler: *"I deliberately do not need this value—please don't give me an unused variable warning!"*

---

## 4. Superpower: Returning Multiple Values from Functions!

A Rust function returns one value. That value can be a tuple containing **2, 3, or more pieces of data**:

```rust
// A function that returns BOTH the minimum and maximum numbers:
fn get_min_and_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    // Call the function and unpack its results immediately:
    let (smallest, largest) = get_min_and_max(42, 17);

    println!("Smallest: {}", smallest);
    println!("Largest:  {}", largest);
}
```

### 📋 Expected Terminal Output:
```text
Smallest: 17
Largest:  42
```

---

## 5. Mutability When Destructuring

You can even make specific unpacked variables mutable by adding `mut` in front of their name:

```rust
fn main() {
    let dimensions = (10, 20);

    // Make 'width' mutable, but keep 'height' immutable:
    let (mut width, height) = dimensions;

    width += 5; // Perfectly valid!
    // height += 5; // Error! height is immutable

    println!("Width: {}, Height: {}", width, height);
}
```

---

## 6. Common Beginner Traps ⚠️

### Trap 1: Mismatched Number of Elements
The pattern on the left **must match the exact count** of items in the tuple:
```rust
let point = (10, 20, 30);
let (x, y) = point; // ❌ COMPILER ERROR: expected a tuple with 3 elements, found one with 2
```
To fix it, either unpack all 3 variables, or use `_` for the ones you don't need:
```rust
let (x, y, _) = point; // ✅ Correct!
```

---

## 7. Quick Recap Checklist
- [x] Destructuring unpacks tuple elements into individual variables: `let (a, b) = pair;`.
- [x] Use `_` to ignore elements you don't need without triggering warnings.
- [x] Functions can return multiple values by returning a tuple `-> (TypeA, TypeB)`.
- [x] You can mark individual unpacked variables as `mut`: `let (mut x, y) = pair;`.

---

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `let (a, b) = pair` | Destructure by position |
| `_` | Ignore this position |
| `as f64` | Convert the sum before division |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_32_tuple_destructuring/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_32_tuple_destructuring/TASK.md).
* Next lesson: [Lesson 33: Fixed-Size Lists: Arrays](33_arrays.md).
