# Detailed Explanation: Task 80 (MPSC Message Channels)

---

### 1. `mpsc::channel()`
```rust
let (tx, rx) = mpsc::channel();
```
* **Multiple Producer, Single Consumer**: Creates an asynchronous channel where multiple senders (`tx`) can stream messages to a single receiver (`rx`).
* **Zero Shared Memory**: Threads do not mutate shared state; they communicate by sending owned messages through the pipe.

---

### 2. Transmitter Cloning (`tx.clone()`)
```rust
let tx1 = tx.clone();
thread::spawn(move || {
    tx1.send(String::from("...")).unwrap();
});
```
* Each worker thread receives its own owned copy of the transmitter `tx`.
* Calling `tx1.send(...)` transfers ownership of the `String` into the channel.

---

### 3. The `drop(tx)` Requirement
```rust
drop(tx);
for msg in rx {
    println!("   [HUB RECEIVE] {}", msg);
}
```
* The `for msg in rx` loop automatically continues until **all** transmitters are dropped.
* If the main thread did not `drop(tx)` its own original transmitter, `rx` would hang waiting forever because it believes the main thread might still send another message!
* Dropping `tx` allows the loop to cleanly finish as soon as worker 1, 2, and 3 finish their work.

