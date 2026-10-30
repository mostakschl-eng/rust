# Task 71: Universal Cargo Scanner (Generic Functions <T>)

## 🎯 What is Your Task?
Write a Rust program to inspect different types of cargo items using a single generic function:
1. Define a generic function:
   ```rust
   fn inspect_cargo<T: std::fmt::Debug>(category: &str, item: T)
   ```
   - Prints `"[CARGO SCANNER: <category>] Item Details: <item>"` using `{:?}`.
2. In `main()`, call `inspect_cargo` with four different types:
   - Category `"Hardware ID"`, integer `1048`
   - Category `"Core Temperature"`, float `98.6`
   - Category `"Mission Codename"`, string slice `"APOLLO-11"`
   - Category `"Safety Clearance"`, boolean `true`

---

## 📋 Expected Output on Your Screen
```text
=== UNIVERSAL CARGO SCANNER ===
[CARGO SCANNER: Hardware ID] Item Details: 1048
[CARGO SCANNER: Core Temperature] Item Details: 98.6
[CARGO SCANNER: Mission Codename] Item Details: "APOLLO-11"
[CARGO SCANNER: Safety Clearance] Item Details: true
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_71_generics_functions/src/main.rs).
2. Declare `<T: std::fmt::Debug>`.
3. Call with integer, float, string slice, and boolean.
4. Test:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_71_generics_functions
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_71_generics_functions/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_71_generics_functions/solution/explain.md)

