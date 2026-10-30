# Detailed Explanation: Task 80 (MPSC Message Channels)

---

### 1. `mpsc::channel()`
```rust
let (tx, rx) = mpsc::channel();
```
* **Multiple Producer, Single Consumer**: Creates an asynchronous channel where multiple senders (`tx`) can stream messages to a single receiver (`rx`).
* **Owned Messages**: Application code here sends owned strings rather than directly mutating one shared string. The channel implementation itself uses shared synchronization internally.

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
* The `for msg in rx` loop automatically continues until **all** transmitters are dropped and queued messages are drained.
* If the main thread did not `drop(tx)` its own original transmitter, `rx` would hang waiting forever because it believes the main thread might still send another message!
* Dropping `tx` allows the loop to cleanly finish after each worker drops its sender and the receiver drains the queued messages. Receiving everything alone is not the same as joining workers.


## Reading the Result Carefully

Several sensor stations post parcels into one mailbox. The recipient finishes only when every sender is gone and all queued parcels have been read.

| Symbol or term | Meaning |
|---|---|
| `mpsc` | multiple producers, single consumer |
| `tx.clone()` | another sender for the same channel |
| `.send()` | transfer message |
| `for msg in rx` | receive until disconnected |

The iterator ends after all senders drop AND queued messages drain. send fails if the receiver is gone; recv fails when disconnected and empty. Keeping a sender alive can block forever; ordering across producers varies.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.

The complete solution retains and joins all three workers, then asserts exactly three received messages before reporting success. Joining observes any worker panic; the channel closing alone does not report why a sender disappeared. The receive order across the three workers remains unspecified.
