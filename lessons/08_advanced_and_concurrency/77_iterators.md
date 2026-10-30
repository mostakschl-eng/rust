# Lesson 77: Iterators: Transforming Data (`map`, `filter`, `collect`)

In Lesson 29 and 37, we looped through collections using traditional `for` loops.  
While `for` loops work, modern idiomatic Rust heavily favors functional data processing pipelines called **Iterators**! 🏭🔄

---

## 1. The Real-Life Analogy: The Factory Assembly Conveyor 🏭

Think of an automated soda bottling plant:
1. **Source (`.iter()`):** Bottles enter the conveyor belt one by one.
2. **Filter (`.filter()`):** A sensor rejects bottles that have cracks or flaws.
3. **Transform (`.map()`):** A nozzle fills each remaining clean bottle with soda.
4. **Package (`.collect()`):** An automated arm boxes the filled bottles into a crate ready for delivery!

In Rust:
> An **Iterator** is responsible for producing a sequence of values lazily.  
> We chain methods like `.map()` and `.filter()` to build elegant data pipelines with zero performance overhead!

---

## 2. The Core Iterator Pipeline in Action

Look at how concise, expressive, and readable this code is:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Build a pipeline:
    // 1. .into_iter(): stream numbers
    // 2. .filter(): keep only even numbers
    // 3. .map(): square each number
    // 4. .collect(): assemble into a new Vec<i32>
    let even_squares: Vec<i32> = numbers
        .into_iter()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * n)
        .collect();

    println!("Original even numbers squared: {:?}", even_squares);
}
```

### 📋 Expected Terminal Output:
```text
Original even numbers squared: [4, 16, 36, 64, 100]
```

---

## 3. Iterators Are Lazy! 😴

In Rust, iterators do **NOTHING** until you actually call a "consuming adapter" (like `.collect()`, `.sum()`, or loop over them):

```rust
let v = vec![1, 2, 3];

// ⚠️ Warning: Does nothing! Rust won't evaluate map until requested!
let stream = v.iter().map(|x| x + 1); 

// Calling .sum() consumes the iterator and does the actual calculation:
let total: i32 = stream.sum();
println!("Sum: {}", total); // 9
```

---

## 4. The Three Flavors of Iterators

| Method | What It Yields | Ownership State |
|---|---|---|
| `v.iter()` | Immutable references `&T` | Borrows elements; `v` remains intact |
| `v.iter_mut()` | Mutable references `&mut T` | Borrows elements with write permissions |
| `v.into_iter()` | Owned values `T` | Consumes and destroys `v` |

---

## 5. Common Iterator Adapters

| Adapter | What It Does | Example |
|---|---|---|
| `.map(f)` | Transforms each element | `.map(\|x\| x * 2)` |
| `.filter(p)` | Keeps items matching predicate | `.filter(\|&x\| x > 10)` |
| `.take(n)` | Takes first `n` elements | `.take(3)` |
| `.skip(n)` | Skips first `n` elements | `.skip(2)` |
| `.sum()` | Sums all numeric elements | `v.iter().sum()` |
| `.collect()` | Gathers into a collection (`Vec`, `HashSet`) | `.collect::<Vec<_>>()` |

---

## 6. Quick Recap Checklist
- [x] Iterators produce a sequence of items lazily.
- [x] Transform streams using `.map()`, filter using `.filter()`.
- [x] Turn iterator streams back into collections using `.collect()`.
- [x] Choose between `.iter()`, `.iter_mut()`, and `.into_iter()`.
- [x] Rust iterators are "zero-cost abstractions" — often faster than manual `for` loops due to compiler loop unrolling!

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_77_iterators/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_77_iterators/TASK.md).
* Next lesson: [Lesson 78: Smart Pointers: Unique Heap Allocation (`Box<T>`)](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/78_box_smart_pointer.md).
