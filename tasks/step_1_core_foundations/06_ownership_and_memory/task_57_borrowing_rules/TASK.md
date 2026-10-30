# Task 57: Audit Ledger Synchronizer (Golden Borrowing Rule)

## 🎯 What is Your Task?
Write a Rust program demonstrating the Golden Borrowing Rule (*Many Readers OR One Writer, never both simultaneously*):
1. Create a mutable ledger:
   ```rust
   let mut audit_log = String::from("[Entry 1: System Boot]");
   ```
2. **Phase 1: Multi-Reader Inspection**
   - Create two simultaneous immutable references: `r1 = &audit_log;` and `r2 = &audit_log;`.
   - Print `"Inspector A: <r1>"` and `"Inspector B: <r2>"`.
   - *Ensure both readers finish their work here so their borrow expires!*
3. **Phase 2: Single-Writer Update**
   - Create a mutable reference: `let writer = &mut audit_log;`.
   - Append `" -> [Entry 2: Backup Verified]"` to `writer`.
   - Print `"Lead Auditor updated log: <writer>"`.
4. **Phase 3: Final Verification Reader**
   - Create a final immutable reference `let final_reader = &audit_log;`.
   - Print `"Final Log State: <final_reader>"`.

---

## 📋 Expected Output on Your Screen
```text
=== AUDIT LEDGER SYNCHRONIZER ===
[Phase 1: Multi-Reader Inspection]
Inspector A: [Entry 1: System Boot]
Inspector B: [Entry 1: System Boot]

[Phase 2: Single-Writer Update]
Lead Auditor updated log: [Entry 1: System Boot] -> [Entry 2: Backup Verified]

[Phase 3: Final Verification]
Final Log State: [Entry 1: System Boot] -> [Entry 2: Backup Verified]
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Schedule readers first, followed by the writer, followed by the final reader.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_57_borrowing_rules
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Print both inspectors before creating the writer.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Use the writer for its append and print, then stop using it before creating the final reader.

</details>

**Before running:** The rule concerns overlapping active access to the same data, not one reference per whole function. Disjoint fields may be borrowed independently.
