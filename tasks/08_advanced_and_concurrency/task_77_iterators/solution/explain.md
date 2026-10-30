# Detailed Explanation: Task 77 (Iterators)

---

### 1. `.iter()` vs `.into_iter()`
```rust
scores.iter()
```
* Using `.iter()` creates an iterator that yields borrowed references (`&i32`).
* This allows us to keep `scores` around and print it afterwards with `println!("Original Scores: {:?}", scores);`. If we used `.into_iter()`, `scores` would be consumed and moved!

---

### 2. The Filter & Map Stages
```rust
.filter(|&&s| s >= 70)
.map(|&s| s + 5)
```
* **`filter(|&&s| s >= 70)`**:
  Because `.iter()` yields references, `filter` passes `&item` to its predicate closure. That means the parameter is a reference to a reference (`&&i32`). Matching `&&s` dereferences it directly so `s` is an `i32` we can compare.
* **`map(|&s| s + 5)`**:
  Unpacks the reference `&s` into an owned copy and adds `5`.

---

### 3. Consuming Adapters: `.collect()` and `.sum()`
```rust
let honors_scores: Vec<i32> = ... .collect();
let total_honors: i32 = honors_scores.iter().sum();
```
* Iterators in Rust are completely lazy. They do not compute anything until a consuming method is invoked.
* `.collect()` runs the pipeline and allocates elements into a `Vec<i32>`.
* `.sum()` iterates through `honors_scores` and adds them all up.

