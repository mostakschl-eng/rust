# Task 61: The Null-Free Profile Register (No NULL in Rust)

## 🎯 What is Your Task?
Write a Rust program to register a user profile with optional fields safely using `Option<String>`:
1. Define a struct `UserProfile`:
   - `username: String`
   - `email: String`
   - `middle_name: Option<String>`
   - `phone_number: Option<String>`
2. In `main()`, instantiate `user`:
   - `username`: `"Farhan"`
   - `email`: `"farhan@tech.com"`
   - `middle_name`: `None`
   - `phone_number`: `Some(String::from("+880-1711-000000"))`
3. Print:
   - Username and Email directly.
   - Middle Name using `match`: if `Some(m)`, print `"Middle Name: <m>"`; if `None`, print `"Middle Name: [Not Provided]"`.
   - Phone Number using `match`: if `Some(p)`, print `"Phone:       <p>"`; if `None`, print `"Phone:       [Not Provided]"`.

---

## 📋 Expected Output on Your Screen
```text
=== USER REGISTRATION PROFILE ===
Username:    Farhan
Email:       farhan@tech.com
Middle Name: [Not Provided]
Phone:       +880-1711-000000
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_61_no_null_in_rust/src/main.rs).
2. Define the struct with `Option<String>` for optional fields.
3. Use `match` to unwrap safely.
4. Test:
   ```bash
   cd tasks/07_errors_and_io/task_61_no_null_in_rust
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_61_no_null_in_rust/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_61_no_null_in_rust/solution/explain.md)

