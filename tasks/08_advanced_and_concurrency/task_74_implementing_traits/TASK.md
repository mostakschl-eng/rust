# Task 74: Geometric Area Comparator (Trait Bounds & where Clauses)

## 🎯 What is Your Task?
Write a Rust program to compare different geometric shapes using explicit Trait Bounds and a `where` clause:
1. Define a trait:
   ```rust
   pub trait Area {
       fn area(&self) -> f64;
   }
   ```
2. Define:
   - `struct Circle { radius: f64 }`
   - `struct Rectangle { width: f64, height: f64 }`
3. Implement `Area` for both structs.
4. Define a comparator function using a `where` clause:
   ```rust
   fn compare_shapes<T, U>(shape_a: &T, shape_b: &U)
   where
       T: Area,
       U: Area,
   ```
   - Computes both areas.
   - Prints `"Shape A Area: <area_a> sq units"`.
   - Prints `"Shape B Area: <area_b> sq units"`.
   - Compares and prints whether Shape A or Shape B is larger.
5. In `main()`, compare a Circle of radius `5.0` with a Rectangle of `10.0 x 8.0`!

---

## 📋 Expected Output on Your Screen
```text
=== GEOMETRIC SHAPE COMPARATOR ===
Shape A (Circle):    78.54 sq units
Shape B (Rectangle): 80.00 sq units
Result: Shape B is larger! 📐
==================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_74_implementing_traits/src/main.rs).
2. Implement the `Area` trait for Circle and Rectangle.
3. Write `compare_shapes` with the `where` clause.
4. Test:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_74_implementing_traits
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_74_implementing_traits/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_74_implementing_traits/solution/explain.md)

