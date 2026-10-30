# Lesson 35: Dynamic Lists: Vectors (`Vec<T>`, `vec![]`)

In Lesson 33, we learned about **Arrays**. Arrays have a fixed number of elements:
> **Arrays have a fixed size that can NEVER change.**

If you create an array of 3 elements, you can never add a 4th element.  
What if you are building:
* A shopping cart where users add items one by one?
* A music playlist where songs are added dynamically?
* A chat app where new incoming messages arrive every second?

You don't know ahead of time how many items you'll have!  
For this, Rust gives us the king of all collections: **Vectors** (`Vec<T>`)! 🛒

---

## 1. The Real-Life Analogy: The Elastic Shopping Cart 🛒

Imagine a grocery shopping cart made of flexible elastic material:
* When you walk into the store, the cart starts **empty**.
* You grab an apple: you toss it in. The cart expands.
* You grab milk, bread, cheese: the cart continues expanding automatically!
* If you decide you don't want the cheese, you pull it out.

In Rust:
> A **Vector** (`Vec<T>`) is a growable, resizable list stored on the **Heap** memory.  
> It can expand or shrink anytime while your program is running!  
> Just like arrays, all elements inside a single vector must have the **same data type**.

---

## 2. Two Ways to Create a Vector

### Method A: Creating an Empty Vector with `Vec::new()`
```rust
fn main() {
    // Create an empty vector that will hold 32-bit integers:
    let mut numbers: Vec<i32> = Vec::new();

    // Push numbers into it:
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("Numbers: {:?}", numbers);
}
```

### Method B: The Super Convenient `vec![]` Macro
If you already have initial values, Rust gives you the convenient `vec![]` macro:
```rust
fn main() {
    let cities = vec!["Tokyo", "Berlin", "New York"];
    println!("Cities: {:?}", cities);
}
```

---

## 3. How Vectors Work Under the Hood 🧠

Vectors consist of two parts:
1. **A small controller (often on the stack for a local variable):**
   * A **pointer** (the memory address where the data lives).
   * **Length (`len`)**: How many items are currently in the vector.
   * **Capacity (`capacity`)**: How many items the vector can hold before needing to reallocate more memory on the heap.
2. **The actual data on the Heap:**
   * A contiguous block of memory where your elements sit side by side!

```text
STACK (Fast Controller)            HEAP (Flexible Storage)
+-----------------------+          +-----------------------+
| Pointer: 0x7FFF00     | -------> | Index 0: 10           |
| Length:   3           |          | Index 1: 20           |
| Capacity: 4           |          | Index 2: 30           |
+-----------------------+          | [Free Slot]           |
                                   +-----------------------+
```

When the vector runs out of capacity, Rust automatically:
1. Requests more storage from the allocator.
2. May relocate the existing elements to a larger allocation. This moves storage; it does not call `.clone()` on the elements.
3. Releases the old allocation if it moved.
The allocator may also grow storage in place. Do not rely on a particular growth factor.
All of this happens completely automatically without you having to do anything!

---

## 4. Reading Elements from a Vector

Just like arrays, vectors support both **direct indexing** and the **safe `.get()` method**:

```rust
fn main() {
    let planets = vec!["Mercury", "Venus", "Earth", "Mars"];

    // 1. Direct indexing:
    println!("First planet:  {}", planets[0]);
    println!("Third planet:  {}", planets[2]);

    // 2. Safe .get():
    match planets.get(3) {
        Some(planet) => println!("Found: {}", planet),
        None => println!("Planet index not found!"),
    }
}
```

### 📋 Expected Terminal Output:
```text
First planet:  Mercury
Third planet:  Earth
Found: Mars
```

---

## 5. Array vs Vector: When to Use Which?

| Feature | Array `[T; N]` | Vector `Vec<T>` |
|---|---|---|
| **Size** | Fixed at compile time | Dynamic & growable at runtime |
| **Memory** | Elements inline with the array | Element buffer on the heap; controller stored with its owner |
| **Allocation** | No separate allocation for its elements | May allocate or reallocate when growing |
| **Best For** | Coordinates `[x, y]`, days of week, RGB colors `[r, g, b]` | User lists, database results, file lines, shopping carts |

---

## 6. Common Beginner Traps ⚠️

### Trap: Forgetting `mut` When Pushing Items
Vectors are immutable by default:
```rust
let numbers = Vec::new();
numbers.push(1); // ❌ COMPILER ERROR: cannot borrow immutable local variable `numbers` as mutable
```
Fix: Add `mut`!
```rust
let mut numbers: Vec<i32> = Vec::new(); // ✅ explicit type for an empty vector
```

---

## 7. Quick Recap Checklist
- [x] Vectors (`Vec<T>`) are resizable, dynamic lists stored on the heap.
- [x] Create an empty vector using `Vec::new()`.
- [x] Create a vector with initial values using `vec![val1, val2, ...]`.
- [x] Add elements to the end using `.push(val)`.
- [x] Access elements with `v[index]` or safely with `v.get(index)`.

---

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `Vec<i32>` | Vector containing i32 elements |
| `Vec::new()` | Create an empty vector |
| `vec![...]` | Create a vector with initial elements |
| `.push(x)` | Append an element |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_35_vectors/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_35_vectors/TASK.md).
* Next lesson: [Lesson 36: Modifying Vectors (`push`, `pop`, `len`)](36_vector_operations.md).
