# Detailed Explanation: Task 78 (Smart Pointers & Box<T>)

---

### 1. Heap Allocation with `Box::new()`
```rust
let boxed_val = Box::new(100);
println!("Heap Allocated Integer:  {}", *boxed_val);
```
* Standard variables are placed directly in the function's stack frame.
* `Box::new(100)` requests space on the heap, places `100` there, and keeps an 8-byte address pointer on the stack.
* Dereferencing with `*boxed_val` accesses the heap value.

---

### 2. Breaking Infinite Recursion with `Box<T>`
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```
* Without `Box<List>`, Rust cannot determine the size of `List` at compile time because each `List` variant contains another `List` recursively.
* With `Box<List>`, the size of `Box` is known and fixed (8 bytes on 64-bit platforms).
* Therefore, the compiler knows exactly how much memory a `List` requires: the size of `i32` (4 bytes) + padding + pointer (8 bytes).

