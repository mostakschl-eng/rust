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

