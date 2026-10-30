# Lesson 72: Generic Structs & Implementation Blocks

In Lesson 71, we created **generic functions**.  
Can we also create **generic data structures**?

Yes! Think of:
* A 2D Coordinate `Point`: Sometimes you want integer pixel coordinates `Point { x: 10, y: 20 }`, and sometimes you want decimal GPS coordinates `Point { x: 23.754, y: 90.378 }`.
* A Network `Response<T>`: Sometimes the payload is a `User`, sometimes it's a `Vec<Product>`.

In Rust, we create these using **Generic Structs**! 📐✨

---

## 1. Defining a Generic Struct: `struct Point<T>`

Place the `<T>` directly after the struct name:

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // Both x and y are integers (T = i32):
    let integer_point = Point { x: 5, y: 10 };

    // Both x and y are floating-point numbers (T = f64):
    let float_point = Point { x: 1.5, y: 4.8 };

    println!("Integer point: {:?}", integer_point);
    println!("Float point:   {:?}", float_point);
}
```

### 📋 Expected Terminal Output:
```text
Integer point: Point { x: 5, y: 10 }
Float point:   Point { x: 1.5, y: 4.8 }
```

> [!WARNING]
> Because both `x` and `y` are of type `T`, they **must have the same type**!  
> Writing `Point { x: 5, y: 4.8 }` will fail because `5` is an integer and `4.8` is a float.

---

## 2. Supporting Different Types for Fields: `struct Point<T, U>`

If you want `x` and `y` to have potentially different types, declare two generic parameters:

```rust
#[derive(Debug)]
struct Coordinate<T, U> {
    x: T,
    y: U,
}

fn main() {
    // x is an integer, y is a float:
    let mixed = Coordinate { x: 5, y: 4.8 };

    // x is text, y is a boolean:
    let custom = Coordinate { x: "North", y: true };

    println!("Mixed:  {:?}", mixed);
    println!("Custom: {:?}", custom);
}
```

---

## 3. Implementing Methods on Generic Structs: `impl<T> Point<T>`

When attaching methods to a generic struct, you must declare `<T>` right after `impl` so Rust knows `T` is a generic type parameter:

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 1. Methods available for ANY type T:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 2. Specialized methods available ONLY for floating-point points (f64):
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5.0, y: 12.0 };

    println!("X coordinate: {}", p.x());
    println!("Distance:     {:.1}", p.distance_from_origin());
}
```

### 📋 Expected Terminal Output:
```text
X coordinate: 5
Distance:     13.0
```

Notice how `distance_from_origin()` is only compiled for `Point<f64>`! An integer `Point<i32>` will not have that method. This is the pinnacle of type-safe modularity!

---

## 4. Quick Recap Checklist
- [x] Generic structs look like `struct Name<T> { field: T }`.
- [x] Use multiple parameters `<T, U>` when fields can have distinct types.
- [x] Declare `impl<T> StructName<T>` to write methods for all types.
- [x] Implement specialized methods for specific concrete types (e.g. `impl StructName<f64>`).

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_72_generic_structs/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_72_generic_structs/TASK.md).
* Next lesson: [Lesson 73: Traits: Defining Shared Behavior](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/73_traits_basics.md).
