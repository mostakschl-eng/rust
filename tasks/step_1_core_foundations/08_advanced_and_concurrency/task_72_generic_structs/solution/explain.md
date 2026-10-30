# Solution Explanation: Task 72 (Generic Structs)

Let's dissect how generic structs store diverse combinations of types safely!

---

## 🔍 The Solution Code

```rust
#[derive(Debug)]
struct Coordinate<T, U> {
    x: T,
    y: U,
}

impl<T, U> Coordinate<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &U { &self.y }
}
```

---

## 🧩 Key Takeaways

1. **`impl<T, U> Coordinate<T, U>`:**
   * The `<T, U>` after `impl` tells Rust we are defining methods generic over two types.
   * The `Coordinate<T, U>` tells Rust which generic type we are implementing methods for.
2. **Getter References (`&T`, `&U`):**
   * Returning references prevents moving fields out of the struct when callers inspect them.


## Reading the Result Carefully

A two-slot organizer can hold two kinds of item, but each individual organizer has fixed slot types once constructed.

| Symbol or term | Meaning |
|---|---|
| `Coordinate<T, U>` | two field types |
| `impl<T, U>` | generic implementation |
| `Self` | current concrete type |
| `&T` | borrow field |

T and U may be the same or different types. One generic instance does not change its field types at runtime; a borrowed getter cannot outlive the container.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
