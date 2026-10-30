# Detailed Explanation: Task 79 (OS Threads & Concurrency)

---

### 1. `thread::spawn(move || { ... })`
* **Real Operating System Thread**: `thread::spawn` tells the host operating system to spawn a brand new native thread that runs concurrent with the main program.
* **The `move` keyword**:
  The background thread might run longer than the local stack of `main()`. Because of this, the compiler will not allow borrowing `dataset` with `&`.
  Writing `move ||` transfers full ownership of `dataset` into the thread's memory, ensuring total safety without data races.

---

### 2. Returning Values via `JoinHandle`
```rust
let (sum, max) = handle.join().unwrap();
```
* **Synchronization**: `handle.join()` blocks the main thread until the spawned worker thread completes.
* **Result Delivery**: Whatever the thread closure returns (in this case, the tuple `(sum, max)`) is packaged into a `Result<T, Box<dyn Any>>`.
* Calling `.unwrap()` extracts the returned tuple directly!

