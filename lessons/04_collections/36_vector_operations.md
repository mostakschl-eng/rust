# Lesson 36: Modifying Vectors (`push`, `pop`, `len`, `is_empty`)

Now that you know what a Vector is, let's learn how to manipulate it like a pro: adding items, removing items, checking size, and managing its contents! 🛠️

---

## 1. The Real-Life Analogy: The Cafeteria Tray Dispenser 🍽️

Think of a spring-loaded tray dispenser in a cafeteria:
* **Push:** The kitchen staff places clean trays on top one by one. The stack grows taller.
* **Pop:** Hungry students pick the top tray off. The stack shrinks.
* **Inspect:** You can glance at the stack to see how many trays are left (`len()`) or check if it's completely empty (`is_empty()`).

This is how vectors work: adding and removing from the end is lightning fast!

---

## 2. Essential Vector Operations

Let's look at a complete example showing all four core operations:

```rust
fn main() {
    let mut inventory = Vec::new();

    // 1. push(): Add items to the end
    inventory.push("Sword");
    inventory.push("Shield");
    inventory.push("Potion");
    println!("Inventory after pushes: {:?}", inventory);

    // 2. len(): Check the current count
    println!("Item count: {}", inventory.len());

    // 3. is_empty(): Check if the vector has 0 items
    println!("Is bag empty? {}", inventory.is_empty());

    // 4. pop(): Remove the last item and return it
    let popped_item = inventory.pop();
    println!("Removed item: {:?}", popped_item);
    println!("Inventory after pop: {:?}", inventory);
}
```

### 📋 Expected Terminal Output:
```text
Inventory after pushes: ["Sword", "Shield", "Potion"]
Item count: 3
Is bag empty? false
Removed item: Some("Potion")
Inventory after pop: ["Sword", "Shield"]
```

---

## 3. Why Does `.pop()` Return `Some` or `None`?

Notice in the output above:
```text
Removed item: Some("Potion")
```
Why didn't `.pop()` just return `"Potion"` directly?  
Because **what if the vector was already empty?**

If a vector is empty, you cannot pop an item!  
Instead of crashing or returning a dangerous null value, Rust returns an **`Option<T>`**:
* If there was an item: it returns **`Some(item)`**.
* If the vector was empty: it returns **`None`**!

```rust
fn main() {
    let mut stack: Vec<i32> = Vec::new();

    match stack.pop() {
        Some(val) => println!("Popped number: {}", val),
        None => println!("The stack was empty! Nothing to pop."),
    }
}
```
**Output:**
```text
The stack was empty! Nothing to pop.
```
This is Rust's safety shield protecting you from crashes!

---

## 4. More Useful Vector Methods

| Method | What It Does | Example |
|---|---|---|
| `v.push(item)` | Appends `item` to the back | `v.push(42);` |
| `v.pop()` | Removes and returns the last item (`Option<T>`) | `let last = v.pop();` |
| `v.len()` | Returns the number of items (`usize`) | `v.len()` |
| `v.is_empty()` | Returns `true` if `len == 0` | `if v.is_empty() { ... }` |
| `v.insert(index, item)` | Inserts `item` at a specific position | `v.insert(0, "First");` |
| `v.remove(index)` | Removes and returns item at `index` | `v.remove(1);` |
| `v.clear()` | Removes all elements from the vector | `v.clear();` |
| `v.contains(&item)` | Checks if an item exists in the vector | `v.contains(&"Sword")` |

---

## 5. Removing All Items: `v.clear()`

```rust
fn main() {
    let mut tasks = vec!["Wash dishes", "Study Rust", "Sleep"];
    println!("Before clear: {} tasks", tasks.len());

    tasks.clear(); // Empty everything out!

    println!("After clear:  {} tasks (Empty: {})", tasks.len(), tasks.is_empty());
}
```
**Output:**
```text
Before clear: 3 tasks
After clear:  0 tasks (Empty: true)
```

---

## 6. Common Beginner Traps ⚠️

### Trap: Calling `.remove(index)` on an Invalid Index
Unlike `.pop()` which safely returns `None` on an empty vector, `.remove(index)` **panics** if the index does not exist!
Always ensure `index < v.len()` before calling `.remove(index)`.

---

## 7. Quick Recap Checklist
- [x] `.push(item)` adds to the end of a mutable vector.
- [x] `.pop()` removes the last item and returns `Some(val)` or `None`.
- [x] `.len()` tells you how many items are currently stored.
- [x] `.is_empty()` returns a boolean `true` or `false`.
- [x] `.clear()` wipes all items from the vector.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/04_collections/task_36_vector_operations/TASK.md`](file:///d:/rust-learning/tasks/04_collections/task_36_vector_operations/TASK.md).
* Next lesson: [Lesson 37: Iterating Over Vectors with `for` Loops](file:///d:/rust-learning/lessons/04_collections/37_vector_iteration.md).
