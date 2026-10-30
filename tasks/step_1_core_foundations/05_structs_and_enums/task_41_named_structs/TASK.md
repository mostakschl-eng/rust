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

1. Open [`src/main.rs`](src/main.rs).
2. Define the `Student` struct with `#[derive(Debug)]`.
3. Instantiate mutable `student1`.
4. Modify `student1.gpa = 3.85;`.
5. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_41_named_structs
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Define all four field types before creating student1.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Declare `let mut student1`, supply every field, update only gpa, and use `{:#?}` for the final multiline record.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

A struct field type validates representation, not all meaning: u8 permits grades outside the school’s actual range.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
