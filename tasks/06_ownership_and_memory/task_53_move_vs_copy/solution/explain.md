# Solution Explanation: Task 53 (Move vs Copy)

Let's dissect how Rust distinguishes stack copy operations from heap ownership moves!

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
   * Passing `original_num` to `copy_primitive` duplicates 4 bytes onto the function's stack frame.
   * `original_num` in `main` remains untouched and 100% valid.
2. **`String` Implements `Move`:**
   * Passing `parcel` to `consume_string` transfers its 24-byte controller.
   * The function's scope `{}` drops `parcel`'s heap buffer when it returns.
   * Attempting to access `parcel` in `main` afterwards would result in `error[E0382]: use of moved value`.

