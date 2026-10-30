# Task 72: Multi-Type Coordinate Matrix (Generic Structs)

## 🎯 What is Your Task?
Write a Rust program to build a generic 2D coordinate container supporting diverse data types:
1. Define a generic struct:
   ```rust
   #[derive(Debug)]
   struct Coordinate<T, U> {
       x: T,
       y: U,
   }
   ```
2. In `impl<T, U> Coordinate<T, U>`, implement:
   - Associated function `new(x: T, y: U) -> Self`
   - Getter method `x(&self) -> &T`
   - Getter method `y(&self) -> &U`
3. In `main()`:
   - Create integer coordinate `(10, 20)`.
   - Create floating-point GPS coordinate `(23.75, 90.38)`.
   - Create mixed coordinate `("Section-Alpha", 42)`.
   - Print all coordinates and access individual coordinates using the `.x()` and `.y()` getters.

---

## 📋 Expected Output on Your Screen
```text
=== GENERIC COORDINATE MATRIX ===
Integer Grid:  X=10, Y=20
GPS Telemetry: X=23.75, Y=90.38
Mixed Beacon:  X="Section-Alpha", Y=42
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_72_generic_structs/src/main.rs).
2. Declare `struct Coordinate<T, U>` and `impl<T, U> Coordinate<T, U>`.
3. Test:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_72_generic_structs
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_72_generic_structs/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_72_generic_structs/solution/explain.md)

