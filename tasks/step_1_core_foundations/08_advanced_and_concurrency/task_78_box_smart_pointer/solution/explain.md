# Detailed Explanation: Task 78 (Smart Pointers & Box<T>)

---

### 1. Heap Allocation with `Box::new()`
```rust
let boxed_val = Box::new(100);
println!("Heap Allocated Integer:  {}", *boxed_val);
```
* Local values are often pictured in a stack frame; their optimized physical storage is not guaranteed.
* `Box::new(100)` requests space on the heap, places `100` there, and keeps a pointer-sized box handle (commonly 8 bytes on a 64-bit target).
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
* Therefore, the compiler knows exactly how much memory a `List` requires: a finite layout accounting for its variants, tag representation, alignment, and padding; do not assume a simple sum of field sizes.


## Reading the Result Carefully

A chain link holds an address card for the next link, rather than physically containing an endless nesting of all future links.

| Symbol or term | Meaning |
|---|---|
| `Box<T>` | owned indirection |
| `Box::new(v)` | box a value |
| `*box` | dereference |
| `Nil` | list end |

Do not add field widths and assume the enum is 12 bytes. Tags, alignment, padding, target, and layout choices affect size. Sized Box handles are pointer-sized; unsized boxes also carry metadata.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
