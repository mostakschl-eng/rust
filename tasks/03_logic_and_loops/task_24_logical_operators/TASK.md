# Task 24: Flight Boarding Pass & ID Verifier (Logical `&&` and `||`)

## 🎯 What is Your Task?
Write a Rust program simulating an **Airport Flight Boarding Gate Gatekeeper** using `&&` and `||`.

A passenger can board the airplane if:
1. They have a valid boarding ticket: `has_ticket: bool` = `true`
2. **AND** they have either a Passport **OR** a National ID Card:
   - `has_passport: bool` = `false`
   - `has_national_id: bool` = `true`

Write an `if` statement combining these rules:
- Condition: `has_ticket && (has_passport || has_national_id)`
- If true: print `"Gate Open: Welcome aboard flight 707! ✈️"`
- Else: print `"Gate Closed: Missing valid ID or ticket! 🛑"`

---

## 📋 Expected Output on Your Screen
```text
=== AIRPORT BOARDING GATE ===
Ticket Verified:      true
Passport Available:   false
National ID Card:     true
Gate Open: Welcome aboard flight 707! ✈️
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_24_logical_operators/src/main.rs).
2. Write the combined logic: `has_ticket && (has_passport || has_national_id)`.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_24_logical_operators
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Parentheses with && and ||</b></summary>

```rust
if has_ticket && (has_passport || has_national_id) {
    println!("Gate Open: Welcome aboard flight 707! ✈️");
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_24_logical_operators/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_24_logical_operators/solution/explain.md)

