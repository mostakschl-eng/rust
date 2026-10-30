# Solution Explanation: Task 37 (Vector Iteration)

Let's break down the three distinct forms of vector iteration used in this task!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== INVOICE & DISCOUNT ENGINE ===");

    let mut prices = vec![12.0, 45.5, 9.99, 120.0];

    // 1. Shared Borrow Iteration (&prices)
    let mut original_sum = 0.0;
    for price in &prices {
        original_sum += price;
    }
    println!("Original Total: ${:.2}", original_sum);

    println!("\nApplying 10% holiday discount to all items...");
    // 2. Mutable Borrow Iteration (&mut prices)
    for price in &mut prices {
        *price *= 0.90;
    }

    println!("=== DISCOUNTED RECEIPT ===");
    // 3. Enumerated Iteration (.iter().enumerate())
    for (idx, price) in prices.iter().enumerate() {
        println!("Item #{}: ${:.2}", idx + 1, price);
    }

    println!("=================================");
}
```

---

## 🧩 Key Concepts

1. **`for price in &prices`:**
   * Borrows each float as a reference `&f64`.
   * Does not consume or destroy `prices`.
2. **`for price in &mut prices` with `*price *= 0.90;`:**
   * Gives mutable access `&mut f64`.
   * The dereference operator `*` is required to mutate the actual numeric value in place.
3. **`.iter().enumerate()`:**
   * Pairs each element with its sequential index starting at `0`.
   * Adding `1` (`idx + 1`) gives user-friendly 1-based numbering!

