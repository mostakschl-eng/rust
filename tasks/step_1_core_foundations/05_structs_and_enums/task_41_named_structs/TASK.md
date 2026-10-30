# Task 41: Student Registration System (Named Structs)

## 🎯 What is Your Task?
Write a Rust program to manage student academic records:
1. Define a struct `Student` with `#[derive(Debug)]` and fields:
   - `name: String`
   - `grade: u8`
   - `gpa: f64`
   - `is_enrolled: bool`
2. In `main()`:
   - Create a mutable instance `student1`:
     - `name`: `"Amina Khan"`
     - `grade`: `11`
     - `gpa`: `3.60`
     - `is_enrolled`: `true`
   - Print original record details.
   - Update `student1.gpa` to `3.85`.
   - Print updated GPA and pretty-print the full struct using `{:#?}`.

---

## 📋 Expected Output on Your Screen
```text
=== STUDENT ACADEMIC RECORD ===
Student:     Amina Khan
Grade Level: 11
Initial GPA: 3.60
Enrolled?    true

Academic update: GPA increased!
Updated GPA: 3.85
Full Record:
Student {
    name: "Amina Khan",
    grade: 11,
    gpa: 3.85,
    is_enrolled: true,
}
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_41_named_structs/src/main.rs).
2. Define the `Student` struct with `#[derive(Debug)]`.
3. Instantiate mutable `student1`.
4. Modify `student1.gpa = 3.85;`.
5. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_41_named_structs
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_41_named_structs/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_41_named_structs/solution/explain.md)

