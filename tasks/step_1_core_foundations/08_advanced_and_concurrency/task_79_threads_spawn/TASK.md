# Task 79: Parallel Worker Thread (`thread::spawn` & `move`)

## 🎯 What is Your Task?
Spawn a background OS worker thread in Rust, transfer data safely into it using `move`, perform calculations concurrently, and return results back to the main thread:

1. Bring `std::thread` and `std::time::Duration` into scope.
2. In `main()`, create a vector:
   ```rust
   let dataset = vec![10, 20, 30, 40, 50];
   ```
3. Spawn a background thread with `thread::spawn(move || { ... })`:
   - Simulate a short workload by sleeping 100ms (`thread::sleep(Duration::from_millis(100));`).
   - Print `"[Worker] Calculations complete!"`.
   - Calculate `sum` and `max`:
     - `let sum: i32 = dataset.iter().sum();`
     - `let max: i32 = *dataset.iter().max().unwrap_or(&0);`
   - Return the tuple `(sum, max)` from the closure.
4. In `main()`, print:
   - `"[Main] Dispatching background task..."`
   - `"[Main] Waiting for worker thread to complete..."`
5. Call `handle.join().unwrap()` to retrieve `(sum, max)` and print them:
   - `"Sum: {}, Max: {}"`

---

## 📋 Expected Output on Your Screen
```text
=== PARALLEL WORKER THREAD ===
[Main] Dispatching background task...
[Main] Waiting for worker thread to complete...
[Worker] Calculations complete!
Sum: 150, Max: 50
==============================
```

---

The output above is one possible schedule. The two lines after dispatch may appear in either order; sleep does not force their order. Sum and Max print after the worker finishes because main joins it.

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Use `thread::spawn(move || { ... })` to take ownership of `dataset`.
3. Return `(sum, max)` from the closure.
4. Bind `let (sum, max) = handle.join().unwrap();`.
5. Run:
   ```bash
   cd tasks/step_1_core_foundations/08_advanced_and_concurrency/task_79_threads_spawn
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Move dataset into the closure and make the tuple its final expression.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Join the returned handle in main and destructure its successful tuple; the final result must print after join.

</details>

**Before running:** Sleep does not guarantee output order or actual parallelism. join().unwrap() panics if the worker panicked. Static references or scoped threads can borrow safely; ordinary spawn cannot capture a short-lived reference.
