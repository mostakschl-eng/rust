# Task 55: The Read-Only Text Analyzer (Shared References &)

## 🎯 What is Your Task?
Write a Rust program to analyze text by borrowing a shared reference (`&String`):
1. Define a function `analyze_text(text: &String) -> (usize, usize)`:
   - Returns a tuple `(text.len(), text.split_whitespace().count())`.
2. In `main()`:
   - Create `let manifesto = String::from("Rust empowers everyone to build reliable and efficient software.");`
   - Call `analyze_text(&manifesto)` passing a shared reference `&`.
   - Print the byte length and word count.
   - Print the original `manifesto` to prove that borrowing did not move or destroy ownership!
   - Create two simultaneous readers `r1 = &manifesto` and `r2 = &manifesto`, and print both!

---

## 📋 Expected Output on Your Screen
```text
=== SHARED BORROWING TEXT ANALYZER ===
Total Bytes: 64
Total Words: 9
Original Text (Still Alive!):
"Rust empowers everyone to build reliable and efficient software."

Multiple Simultaneous Readers:
Reader 1: Rust empowers everyone to build reliable and efficient software.
Reader 2: Rust empowers everyone to build reliable and efficient software.
======================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Take `&String` parameter in `analyze_text`.
3. Pass `&manifesto` using the `&` operator.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_55_borrowing_shared
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Return a tuple containing byte length and the number of whitespace-separated words.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Pass a shared reference, destructure the tuple, and create both readers without moving manifesto. Prefer &str for a general text-only API.

</details>

**Before running:** String length counts bytes, not letters. The manifesto contains 64 ASCII bytes and 9 whitespace-separated words.
