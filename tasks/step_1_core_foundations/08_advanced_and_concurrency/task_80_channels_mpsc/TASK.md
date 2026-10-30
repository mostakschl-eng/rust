# Task 80: Multi-Sensor Telemetry Hub (MPSC Channels)

## 🎯 What is Your Task?
Implement multi-producer, single-consumer message passing across parallel threads using `std::sync::mpsc`:

1. Bring `std::sync::mpsc`, `std::thread`, and `std::time::Duration` into scope.
2. In `main()`, create a channel:
   ```rust
   let (tx, rx) = mpsc::channel();
   ```
3. Spawn 3 background worker threads representing sensors:
   - **Worker 1**: Uses `let tx1 = tx.clone();` and sends `String::from("Sensor Alpha: Temp 24.5 C")`.
   - **Worker 2**: Uses `let tx2 = tx.clone();` and sends `String::from("Sensor Beta:  Pressure 1013 hPa")`.
   - **Worker 3**: Uses `let tx3 = tx.clone();` and sends `String::from("Sensor Gamma: Humidity 55%")`.
4. In `main()`, `drop(tx)` so the channel closes when the 3 workers finish.
5. Iterate through incoming telemetry packets using:
   ```rust
   for msg in rx {
       println!("   [HUB RECEIVE] {}", msg);
   }
   ```
6. Keep each worker JoinHandle. After receiving, join all three workers and assert that three messages arrived before printing completion status. Print banner lines and completion status!

---

## 📋 Expected Output on Your Screen
```text
=== SPACE STATION TELEMETRY HUB (MPSC) ===
[HUB] Initializing sensor receivers...
   [HUB RECEIVE] Sensor Alpha: Temp 24.5 C
   [HUB RECEIVE] Sensor Beta:  Pressure 1013 hPa
   [HUB RECEIVE] Sensor Gamma: Humidity 55%
[HUB] All telemetry data collected successfully! 🛰️
==========================================
```
*(Note: Sensor order may vary slightly depending on OS thread scheduling)*

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Clone `tx` for each thread before moving it into `thread::spawn(move || { ... })`.
3. Drop the remaining original `tx` handle before reading from `rx`.
4. Run:
   ```bash
   cd tasks/step_1_core_foundations/08_advanced_and_concurrency/task_80_channels_mpsc
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Create one channel and clone its sender before moving one sender into each worker.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Drop the main sender before iterating rx. For full worker-success reporting, keep join handles and check each join after receiving.

</details>

**Before running:** The iterator ends after all senders drop AND queued messages drain. send fails if the receiver is gone; recv fails when disconnected and empty. Keeping a sender alive can block forever; ordering across producers varies.
