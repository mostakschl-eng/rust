# Lesson 51: Computer Memory Demystified: Stack vs Heap

Welcome to **Level 6: Rust's Sacred Core — Ownership & Memory**! 🧠⚡

This is the most famous, powerful, and unique part of Rust.  
Languages use different memory-management strategies. Java and JavaScript use tracing garbage collectors. CPython mainly uses reference counting with a cycle collector. C supports manual allocation; C++ also provides automatic resource management through RAII and smart pointers.

Rust uses ownership and borrowing to prevent many memory errors in safe code without a required tracing garbage collector. Programs can still panic, leak memory, run out of memory, or contain logic bugs. Allocation and cleanup still have runtime costs.

To understand how, we must first understand how computer memory works: **The Stack vs The Heap**.

---

## 1. The Real-Life Analogy: The Neat Desk vs The Big Warehouse 🏢

Imagine you work in an office:

### The Stack (Your Neat Desk):
* Right in front of you on your desk is a neat stack of clipboards.
* When you start a task, you place a clipboard on top.
* When you finish the task, you remove that top clipboard (**Last In, First Out — LIFO**).
* Everything on the desk has a **known, fixed size** (e.g. an 8.5 x 11 sheet of paper).
* Accessing things on your desk is **blazingly fast**!

### The Heap (The Storage Warehouse Across the Street):
* Now imagine someone brings you a massive, unpredictable delivery: a box that might hold 5 items today, or 50,000 items tomorrow.
* It's too big and unpredictable to sit on your desk!
* So you call the warehouse across the street:
  1. The warehouse clerk finds an empty aisle with enough room.
  2. They store your boxes there.
  3. They hand you back a tiny receipt with the aisle address: **"Shelf #402"**.
* You put that **tiny receipt address on your desk**!

---

## 2. Stack vs Heap in Rust Code

Look at how Rust divides data:

```rust
fn main() {
    // 1. STACK ALLOCATION:
    // Fixed size known at compile time:
    let age: i32 = 28;         // 4 bytes directly on the stack
    let price: f64 = 99.50;    // 8 bytes directly on the stack
    let is_logged: bool = true;// 1 byte directly on the stack
    let point: [i32; 2] = [10, 20]; // 8 bytes directly on the stack

    // 2. HEAP ALLOCATION:
    // Dynamic, growable text:
    let mut message = String::from("Hello"); // Heap allocated!
    message.push_str(", World!");

    println!("Stack values: {}, {}, {}", age, price, is_logged);
    println!("Heap string:  {}", message);
}
```

---

The example prints:

```text
Stack values: 28, 99.5, true
Heap string:  Hello, World!
```

The comments picture unoptimized local storage. A value's type alone does not force a physical stack location.

## 3. What Does a `String` Look Like in Memory?

When you create `let s = String::from("Rust");`, Rust stores it in two halves:

```text
LOCAL STRING HANDLE (typically 24 bytes on 64-bit targets)           THE HEAP (Dynamic storage)
+--------------+----------+                      +---+---+---+---+
| Field        | Value    |                      | R | u | s | t |
+--------------+----------+                      +---+---+---+---+
| Pointer (ptr)| 0x1048   | -------------------> | 0 | 1 | 2 | 3 | (Address 0x1048)
| Length (len) | 4 bytes  |                      +---+---+---+---+
| Capacity     | 4 bytes  |
+--------------+----------+
```

* **The handle:** A `String` contains a pointer, byte length, and capacity. It commonly occupies 24 bytes on a 64-bit target. The handle can itself be inside another heap allocation; optimization may keep locals in registers.
* **On the Heap:** The actual letters `'R', 'u', 's', 't'` live on the heap.

---

## 4. Side-by-Side Comparison: Stack vs Heap

| Feature | The Stack 📚 | The Heap 📦 |
|---|---|---|
| **Speed** | Usually cheap allocation for local frames | Allocator cost and access cost depend on the workload |
| **Size Requirement** | Must be fixed and known at compile time | Can grow or shrink dynamically at runtime |
| **Organization** | Strictly ordered: Last In, First Out (LIFO) | Managed by an allocator; allocations need not be adjacent |
| **Examples** | Local numbers and arrays are common examples; these types can also live on the heap | Buffers owned by `String`, `Vec<T>`, `HashMap<K, V>`, and nonzero-sized boxed values |
| **Deallocation** | Local storage is reclaimed as execution leaves its frame | Handled by Rust's Ownership rules! |

---

## 5. The Core Dilemma: When Does the Heap Memory Get Freed?

Local values are normally dropped when their scopes end. Their machine storage and cleanup are implementation details; not every variable has a physical stack slot.

**But what about the heap?**
* A manually managed allocation must be released correctly. C++ RAII and Rust ownership automate much of this responsibility.
* Garbage collection trades runtime tracking and reclamation work for a different programming model.
* In this `String` example, its normal drop releases the buffer to the allocator. The allocator may retain that space for reuse rather than return it to the OS.

We will discover the sacred rules of Ownership in Lesson 52!

---

## 6. Quick Recap Checklist
- [x] The Stack stores fixed-size data in Last-In-First-Out order with near-instant speed.
- [x] The Heap stores dynamic, resizable data; finding space requires an allocator.
- [x] `String` and `Vec<T>` have fixed-size handles and growable buffers; handle size depends on the target, and handles need not live on the stack.
- [x] Owned buffers are normally released by drop, without a required tracing garbage collector.

---


## Symbols and Practical Boundaries

The local handle is a warehouse receipt; the buffer is the stored goods. Moving the receipt need not move all the goods.

| Symbol or term | Meaning |
|---|---|
| `String` | owned UTF-8 text |
| `Vec<T>` | growable sequence |
| `.len()` | used length |
| `.capacity()` | reserved space |

**Common pitfall:** A type is not inherently a stack type or heap type. Local storage may be optimized into registers, and a fixed-size array can be boxed.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_51_stack_vs_heap/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_51_stack_vs_heap/TASK.md).
* Next lesson: [Lesson 52: The Three Sacred Rules of Rust Ownership](52_three_ownership_rules.md).

Further reading: [Rust Book: ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).
