# Task 14: International Badge (Characters & Emojis)

## 🎯 What is Your Task?
Write a Rust program that stores individual letters, symbols, and emojis using the **`char`** type with single quotes `' '`.

Declare these 4 `char` variables:
1. `grade: char` = `'A'`
2. `currency: char` = `'$'`
3. `rocket: char` = `'🚀'`
4. `crab_mascot: char` = `'🦀'`

Print a clean badge displaying each character!

---

## 📋 Expected Output on Your Screen
```text
=== RUST DEVELOPER BADGE ===
Exam Grade:   A
Currency:     $
Launch Icon:  🚀
Official Pet: 🦀 Ferris the Crab!
============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_14_char_and_emojis/src/main.rs).
2. Declare each `char` using single quotes `' '`.
3. Print the badge.
4. Test with:
   ```bash
   cd tasks/02_types_and_math/task_14_char_and_emojis
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Single quotes for emojis</b></summary>

Emojis in Rust are placed inside single quotes just like letters:
```rust
let rocket: char = '🚀';
let crab: char = '🦀';
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_14_char_and_emojis/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_14_char_and_emojis/solution/explain.md)
