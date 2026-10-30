# Detailed Explanation: Task 79 (OS Threads & Concurrency)

---

### 1. `thread::spawn(move || { ... })`
* **Real Operating System Thread**: `thread::spawn` tells the host operating system to spawn a brand new native thread that runs concurrent with the main program.
* **The `move` keyword**:
  The background thread might run longer than the local stack of `main()`. Because of this, the compiler will not allow borrowing `dataset` with `&`.
  Writing `move ||` transfers full ownership of `dataset` into the thread's memory, allowing this owned vector to satisfy the spawn lifetime requirement. Panics and other logic errors remain possible.

---

### 2. Returning Values via `JoinHandle`
```rust
let (sum, max) = handle.join().unwrap();
```
* **Synchronization**: `handle.join()` blocks the main thread until the spawned worker thread completes.
* **Result Delivery**: Whatever the thread closure returns (in this case, the tuple `(sum, max)`) is packaged into a `Result<T, Box<dyn Any + Send + 'static>>`.
* Calling `.unwrap()` extracts the returned tuple directly!


## Reading the Result Carefully

A kitchen dispatches a job to another chef, then waits for the returned tray before serving the result.

| Symbol or term | Meaning |
|---|---|
| `thread::spawn` | start OS thread |
| `move ||` | capture by value |
| `.join()` | wait and get Result |
| `Send` | transferable between threads |

Sleep does not guarantee output order or actual parallelism. join().unwrap() panics if the worker panicked. Static references or scoped threads can borrow safely; ordinary spawn cannot capture a short-lived reference.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
