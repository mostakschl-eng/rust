# Solution Explanation: Task 40 (HashMaps)

Let's dissect how key-value storage works with Rust's standard library `HashMap`!

---

## 🔍 The Solution Code

```rust
use std::collections::HashMap;

fn main() {
    println!("=== FRESH MARKET PRICE CATALOG ===");

    let mut prices = HashMap::new();

    prices.insert("Apple", 2.50);
    prices.insert("Banana", 1.20);
    prices.insert("Mango", 3.75);

    println!("Initial catalog items: {}", prices.len());

    println!("Has 'Banana'? {}", prices.contains_key("Banana"));

    match prices.get("Mango") {
        Some(price) => println!("Lookup [Mango]: Price of Mango: ${:.2}", price),
        None => println!("Sorry, Mango is not in stock!"),
    }

    match prices.get("Dragonfruit") {
        Some(price) => println!("Lookup [Dragonfruit]: Price of Dragonfruit: ${:.2}", price),
        None => println!("Lookup [Dragonfruit]: Sorry, Dragonfruit is not in stock!"),
    }

    println!("Adding Orange via entry API...");
    prices.entry("Orange").or_insert(1.90);

    println!("Updated catalog count: {}", prices.len());

    println!("==================================");
}
```

---

## 🧩 Key Methods Breakdown

1. **`use std::collections::HashMap;`:**
   * Brings the HashMap type from Rust's standard collection library into local scope.
2. **`prices.insert(key, value)`:**
   * Hashes the key and stores the key-value pair in an internal hash table bucket.
3. **`prices.get(&key)`:**
   * Returns an `Option<&Value>`. If found, `Some(&price)`; if absent, `None`.
4. **`prices.entry(key).or_insert(default)`:**
   * Inspects if `key` already exists. If yes, leaves it alone; if absent, inserts `default` in a single efficient operation!

