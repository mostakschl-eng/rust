# Solution Explanation: Task 41 (Named Structs)

Let's dissect how custom named structs define real-world entities in Rust!

---

## 🔍 The Solution Code

```rust
#[derive(Debug)]
struct Student {
    name: String,
    grade: u8,
    gpa: f64,
    is_enrolled: bool,
}

fn main() {
    println!("=== STUDENT ACADEMIC RECORD ===");

    let mut student1 = Student {
        name: String::from("Amina Khan"),
        grade: 11,
        gpa: 3.60,
        is_enrolled: true,
    };

    println!("Student:     {}", student1.name);
    println!("Grade Level: {}", student1.grade);
    println!("Initial GPA: {:.2}", student1.gpa);
    println!("Enrolled?    {}", student1.is_enrolled);

    println!("\nAcademic update: GPA increased!");
    student1.gpa = 3.85;

    println!("Updated GPA: {:.2}", student1.gpa);
    println!("Full Record:\n{:#?}", student1);

    println!("===============================");
}
```

---

## 🧩 Key Takeaways

1. **`#[derive(Debug)]`:**
   * Automatically generates the formatting code so `{:?}` (inline) and `{:#?}` (multiline pretty print) work seamlessly.
2. **Field Access & Mutability:**
   * Declaring `let mut student1` gives mutable access to all fields (`student1.gpa = 3.85;`).
3. **Type Safety:**
   * The compiler ensures every field matches its defined type: `grade` must be an integer, `gpa` a float, and `name` an owned `String`.

