# Lesson 79: Concurrency: Spawning OS Threads (`thread::spawn`)

Modern computers have CPUs with 4, 8, 16, or 64 cores.  
Up until now, all of our code ran on **one single thread**: line 1, then line 2, waiting patiently.

What if you want to:
* Download 10 files simultaneously?
* Process thousands of images in parallel across all CPU cores?
* Run background music while an interactive game loop renders graphics?

**Concurrency** means tasks make progress during overlapping periods. **Parallelism** means work actually runs at the same instant. OS threads can provide either, depending on scheduling and available cores. Start with **`std::thread::spawn`**. 🧵⚡

---

## 1. The Real-Life Analogy: The Restaurant Kitchen Team 👨‍🍳👩‍🍳

* **Single-Threaded:** One chef in the kitchen chops the onions, waits 20 minutes for the soup to simmer, then bakes the bread. Everything happens one after another.
* **Multi-Threaded:**
  * **Chef 1 (Main Thread):** Bakes the bread.
  * **Chef 2 (Spawned Thread):** Simmers the soup.
  * Two chefs may work simultaneously with enough resources. With one stove, they may instead take turns. Both arrangements allow overlapping tasks.

In Rust:
> `std::thread::spawn` creates a real **Operating System thread**.  
> The OS schedules your code across available physical CPU cores!

---

## 2. Spawning Your First Thread in Rust

Look at how simple it is:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new background thread:
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("   [Background Thread] Working on item #{}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Main thread code:
    for i in 1..=3 {
        println!("[Main Thread] Supervising step #{}", i);
        thread::sleep(Duration::from_millis(70));
    }

    // Wait for the background thread to finish:
    handle.join().unwrap();

    println!("All threads finished cleanly! 🏁");
}
```

### 📋 One Possible Terminal Output:
```text
[Main Thread] Supervising step #1
   [Background Thread] Working on item #1
   [Background Thread] Working on item #2
[Main Thread] Supervising step #2
   [Background Thread] Working on item #3
   [Background Thread] Working on item #4
[Main Thread] Supervising step #3
   [Background Thread] Working on item #5
All threads finished cleanly! 🏁
```

The lines can interleave differently on each run. Interleaving alone does not prove parallel execution. Sleeping is not synchronization, and the final line appears after the worker completes because of `join()`.

---

## 3. Why `.join()` is Crucial! 🛑

What happens if the `main` thread finishes while the background thread is still running?  
**When `main()` exits, the entire process terminates!** Any background thread that hasn't finished is instantly killed.

To prevent this:
* `thread::spawn` returns a **`JoinHandle`**.
* Calling `handle.join().unwrap()` tells `main`:
  > *"Pause here and wait for the spawned thread to finish its work before exiting!"*

---

## 4. Passing Data to Threads: The `move` Keyword 📦

What if a background thread needs data from the `main` thread?

`thread::spawn` requires its closure and return value to be `Send + 'static`: they cannot contain short-lived borrowed data. A reference to suitably shareable static data is allowed. `thread::scope` allows borrowing local data because scoped threads are joined before the scope returns. For this ordinary spawn, use **`move`** to transfer the owned vector:

```rust
use std::thread;

fn main() {
    let dataset = vec![10, 20, 30, 40];

    // 'move' moves ownership of 'dataset' into the spawned thread:
    let handle = thread::spawn(move || {
        println!("Background thread calculating sum: {}", dataset.iter().sum::<i32>());
    });

    handle.join().unwrap();
}
```

Rust's type system guarantees **Fearless Concurrency**: data races are caught at compile time!

---

## 5. Quick Recap Checklist
- [x] `thread::spawn(|| { ... })` spawns a native OS thread.
- [x] Use `handle.join().unwrap()` to wait for a thread to finish before exiting.
- [x] Use `move ||` to transfer ownership of variables into spawned threads safely.
- [x] Safe Rust prevents data races through borrowing and thread-safety traits. Threads can still panic, deadlock, or contain ordering bugs; `join()` reports a worker panic as `Err`.

---


## Symbols and Practical Boundaries

A kitchen dispatches a job to another chef, then waits for the returned tray before serving the result.

| Symbol or term | Meaning |
|---|---|
| `thread::spawn` | start OS thread |
| `move ||` | capture by value |
| `.join()` | wait and get Result |
| `Send` | transferable between threads |

**Common pitfall:** Sleep does not guarantee output order or actual parallelism. join().unwrap() panics if the worker panicked. Static references or scoped threads can borrow safely; ordinary spawn cannot capture a short-lived reference.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/08_advanced_and_concurrency/task_79_threads_spawn/TASK.md`](../../../tasks/step_1_core_foundations/08_advanced_and_concurrency/task_79_threads_spawn/TASK.md).
* Next lesson: [Lesson 80: Thread Communication: Message Passing with Channels (`mpsc`)](80_channels_mpsc.md).

Further reading: [Scoped threads](https://doc.rust-lang.org/std/thread/fn.scope.html).
