# Task 58: Alchemist Potion Shelf (Non-Lexical Lifetimes NLL)

## 🎯 What is Your Task?
Write a Rust program demonstrating modern Non-Lexical Lifetimes (NLL):
1. Create a mutable vector of potions:
   ```rust
   let mut shelf = vec!["Health Potion", "Mana Potion"];
   ```
2. Borrow the first potion as an immutable reference:
   ```rust
   let active_potion = &shelf[0];
   ```
3. Print `"Currently inspecting: <active_potion>"`.
4. Right after printing, modify the vector by pushing a new potion:
   ```rust
   shelf.push("Elixir of Life");
   ```
   *(Notice: In pre-2018 Rust, this would fail because `active_potion` lived until the end of `main`. In modern Rust with NLL, `active_potion`'s lifetime ended the moment it was printed!)*
5. Print the full updated shelf using `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== ALCHEMIST POTION SHELF ===
Currently inspecting: Health Potion
Adding new rare potion to shelf...
Shelf Inventory: ["Health Potion", "Mana Potion", "Elixir of Life"]
==============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_58_borrow_checker_nll/src/main.rs).
2. Read from `shelf[0]`, print it, and immediately mutate `shelf.push(...)`.
3. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_58_borrow_checker_nll
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_58_borrow_checker_nll/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_58_borrow_checker_nll/solution/explain.md)

