# Task 79: Parallel Worker Thread (`thread::spawn` & `move`)

## 🎯 What is Your Task?
Spawn a background OS worker thread in Rust, transfer data safely into it using `move`, perform calculations in parallel, and return results back to the main thread:

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

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_79_threads_spawn/src/main.rs).
2. Use `thread::spawn(move || { ... })` to take ownership of `dataset`.
3. Return `(sum, max)` from the closure.
4. Bind `let (sum, max) = handle.join().unwrap();`.
5. Run:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_79_threads_spawn
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_79_threads_spawn/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_79_threads_spawn/solution/explain.md)

