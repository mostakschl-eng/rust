# Task 06: Solution Breakdown & Parameter Explanations

## 1. Line-by-Line Breakdown

```rust
fn print_item(item_name: &str, quantity: i32, price: i32) {
    println!("Item: {:<10} | Qty: {} | Price: ${}", item_name, quantity, price);
}
```

* `item_name: &str` accepts text strings like `"Laptop"`.
* `quantity: i32` accepts whole numbers like `1`.
* `price: i32` accepts whole numbers like `900`.
* Bonus trick: `{:<10}` tells Rust: *"Reserve 10 character spaces and align the text to the left."* This keeps columns lined up evenly!

---

## 2. Parameter Reusability
By writing `print_item` once, you can print 100 or 1,000 grocery items with zero repeated code. This is modular software design!

