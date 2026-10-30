# Solution Explanation: Task 54 (Deep Copying with .clone())

Let's dissect how `.clone()` produces independent heap duplicates!

---

## 🔍 The Solution Code

```rust
#[derive(Debug, Clone)]
struct TaskItem {
    title: String,
    priority: u8,
}

fn main() {
    let mut original_doc = String::from("Draft v1");
    let mut cloned_doc = original_doc.clone();

    original_doc.push_str(" (Archived Copy)");
    cloned_doc.push_str(" (Revised by Editor)");

    let task1 = TaskItem {
        title: String::from("Fix Server Crash"),
        priority: 5,
    };
    let mut task2 = task1.clone();
    task2.priority = 1;
}
```

---

## 🧩 Key Takeaways

1. **Heap Duplication:**
   * `.clone()` allocates new memory on the heap and duplicates all content.
   * `original_doc` and `cloned_doc` point to two completely separate memory addresses.
2. **`#[derive(Clone)]` on Structs:**
   * Generates a `.clone()` implementation that recursively calls `.clone()` on every field (`title.clone()` and copying `priority`).
   * Modifying `task2.priority` does not affect `task1.priority`.

