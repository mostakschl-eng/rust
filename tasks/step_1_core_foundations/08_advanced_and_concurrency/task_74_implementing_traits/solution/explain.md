# Detailed Explanation: Task 74 (Geometric Area Comparator)

---

### 1. Defining the `Area` Trait
```rust
pub trait Area {
    fn area(&self) -> f64;
}
```
* **Contract**: Any type that claims to implement `Area` promises to provide an `area(&self)` method that borrows itself and returns an `f64`.

---

### 2. Implementing the Trait for Concrete Types
```rust
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```
* `Circle` calculates $\pi \cdot r^2$ using `std::f64::consts::PI`.
* `Rectangle` calculates $width \cdot height$.
* Both satisfy the `Area` contract!

---

### 3. The `where` Clause Trait Bound
```rust
fn compare_shapes<T, U>(shape_a: &T, shape_b: &U)
where
    T: Area,
    U: Area,
```
* **Why two generic parameters (`T`, `U`)?**
  If we used `fn compare_shapes<T: Area>(shape_a: &T, shape_b: &T)`, both arguments would have to be the exact same type (e.g. Circle and Circle).
  By using two distinct types `T` and `U`, `shape_a` can be a `Circle` and `shape_b` can be a `Rectangle`!
* **The `where` Clause**:
  Moves type constraint bounds after the signature line, making long function signatures significantly cleaner and easier to read.


## Reading the Result Carefully

A surveyor compares circles and rectangles by asking each for area, without requiring both plots to have the same shape.

| Symbol or term | Meaning |
|---|---|
| `T: Area` | T supports area |
| `U: Area` | independent type |
| `where` | list bounds |
| `+` | require multiple traits |

The generic comparator should label inputs A and B, not assume their concrete shapes. This exercise uses finite nonnegative sample dimensions; production geometry must validate inputs.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
