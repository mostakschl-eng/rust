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

1. Open [`src/main.rs`](src/main.rs).
2. Write the combined logic: `has_ticket && (has_passport || has_national_id)`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_24_logical_operators
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

A ticket is mandatory, while either of the two ID flags is sufficient. Group the alternatives together.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Use AND between the ticket rule and the grouped OR rule. Test a missing ticket with a valid national ID: boarding must fail.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

