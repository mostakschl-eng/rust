# Lesson 37: Iterating Over Vectors with `for` Loops

Now that we have vectors holding lists of numbers, names, or prices, how do we process every single element in that list?  
We use a **`for` loop** to iterate over the collection! 🔄

---

## 1. The Real-Life Analogy: The Supermarket Checkout Belt 🛒

Think of a conveyor belt at the supermarket:
* You place 5 items on the belt: Milk, Bread, Apples, Eggs, Coffee.
* The cashier picks up each item **one by one**:
  * Item 1: Scans and prints price.
  * Item 2: Scans and prints price.
  * ... until the belt is empty!

In Rust:
> **Iterating** means walking through a collection element by element and running your code for each one.

---

## 2. Three Ways to Loop Through a Vector

Rust is very strict about **ownership and borrowing** (which we will explore deeply in Level 6).  
When looping through a vector, you have three options:

1. **Read-Only Borrow (`for item in &v`)**: You just want to look at or print the elements without modifying or destroying the vector.
2. **Mutable Borrow (`for item in &mut v`)**: You want to change the values inside the vector in place.
3. **Taking Ownership (`for item in v`)**: You want to consume and destroy the vector, moving each item out.

Let's see the two most common ones in action!

---

## 3. Method 1: Read-Only Iteration (`&v`)

Use `&` in front of the vector name so you can continue using the vector afterwards:

```rust
fn main() {
    let guests = vec!["Rahim", "Karim", "Ayesha", "Fatima"];

    println!("=== GUEST LIST ===");
    for name in &guests {
        println!("Welcome, {}! 🎉", name);
    }

    // Because we used &guests, the vector is still alive and usable!
    println!("Total attendees: {}", guests.len());
}
```

### 📋 Expected Terminal Output:
```text
=== GUEST LIST ===
Welcome, Rahim! 🎉
Welcome, Karim! 🎉
Welcome, Ayesha! 🎉
Welcome, Fatima! 🎉
Total attendees: 4
```

---

## 4. Method 2: Modifying Elements in Place (`&mut v`)

What if you want to double every number in a vector?  
Use `&mut` on the vector, and dereference with `*` to modify the value:

```rust
fn main() {
    let mut prices = vec![10, 20, 30, 40];
    println!("Original prices: {:?}", prices);

    // Apply a 50% inflation / doubling:
    for price in &mut prices {
        *price *= 2; // Notice the asterisk (*) to update the value behind the reference!
    }

    println!("Doubled prices:  {:?}", prices);
}
```

### 📋 Expected Terminal Output:
```text
Original prices: [10, 20, 30, 40]
Doubled prices:  [20, 40, 60, 80]
```

---

## 5. Getting Both Index and Value: `.iter().enumerate()`

What if you want to print numbered list items, like `1. Apple`, `2. Banana`?  
Rust provides the **`.iter().enumerate()`** helper:

```rust
fn main() {
    let top_movies = vec!["Inception", "Interstellar", "The Dark Knight"];

    for (index, movie) in top_movies.iter().enumerate() {
        println!("{}. {}", index + 1, movie);
    }
}
```

### 📋 Expected Terminal Output:
```text
1. Inception
2. Interstellar
3. The Dark Knight
```

---

## 6. Common Beginner Traps ⚠️

### Trap: Consuming the Vector Accidentally
If you omit the `&` in the `for` loop:
```rust
let v = vec![1, 2, 3];
for x in v { // Moves ownership of v!
    println!("{}", x);
}
println!("{}", v.len()); // ❌ COMPILER ERROR: borrow of moved value `v`
```
**Fix:** Always put `&` in front of the collection name (`for x in &v`) unless you deliberately want to destroy it!

---

## 7. Quick Recap Checklist
- [x] Use `for item in &v` to safely read items without consuming the vector.
- [x] Use `for item in &mut v` with `*item` to modify values in place.
- [x] Use `.iter().enumerate()` to get both the index and the item together.
- [x] Never omit `&` unless you want the vector destroyed after the loop finishes.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/04_collections/task_37_vector_iteration/TASK.md`](file:///d:/rust-learning/tasks/04_collections/task_37_vector_iteration/TASK.md).
* Next lesson: [Lesson 38: Text in Memory: `String` vs `&str`](file:///d:/rust-learning/lessons/04_collections/38_string_vs_str.md).
