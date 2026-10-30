# Solution Explanation: Task 53 (Move vs Copy)

Let's dissect how Rust distinguishes implicit Copy operations from ownership moves!

---

## 🔍 The Solution Code

```rust
fn copy_primitive(val: i32) {
    println!("Inside copy_primitive: {}", val);
}

fn consume_string(text: String) {
    println!("Inside consume_string: {} (Taking full ownership)", text);
}

fn main() {
    let original_num: i32 = 500;
    copy_primitive(original_num);
    println!("Still alive in main:   {} (Copy trait preserved it!)", original_num);

    let parcel = String::from("Rocket Fuel");
    consume_string(parcel);
    println!("Parcel ownership successfully moved and cleaned up.");
}
```

---

## 🧩 Key Takeaways

1. **`i32` Implements `Copy`:**
   * Passing `original_num` to `copy_primitive` copies its integer value. The compiler may use registers or inline the call; this does not guarantee a copy onto a stack frame.
   * `original_num` in `main` remains untouched and 100% valid.
2. **`String` Does Not Implement `Copy`:**
   * Passing `parcel` to `consume_string` transfers its fixed-size handle.
   * The function's scope `{}` drops `parcel`'s heap buffer when it returns.
   * Attempting to access `parcel` in `main` afterwards would result in `error[E0382]: use of moved value`.


## Reading the Result Carefully

Copying a written number makes two notes; handing over a suitcase transfers responsibility for one suitcase.

| Symbol or term | Meaning |
|---|---|
| `Copy` | implicit duplication permission |
| `String` parameter` | ownership transfer |
| `&String` | shared borrowing |

No standard Move trait exists. A non-Copy struct can contain only numbers and still move; location does not decide Copy.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
