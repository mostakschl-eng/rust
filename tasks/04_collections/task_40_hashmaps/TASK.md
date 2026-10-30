# Task 40: Fresh Market Price Catalog (HashMap)

## 🎯 What is Your Task?
Write a Rust program to manage a supermarket fruit price catalog using `HashMap`:
1. Import `HashMap` with:
   ```rust
   use std::collections::HashMap;
   ```
2. Create a mutable `prices: HashMap<&str, f64> = HashMap::new();`.
3. Insert initial items:
   - `"Apple"` -> `2.50`
   - `"Banana"` -> `1.20`
   - `"Mango"` -> `3.75`
4. Check if `"Banana"` is available with `prices.contains_key("Banana")`.
5. Look up `"Mango"` with `prices.get("Mango")` using `match`:
   `"Price of Mango: $3.75"`
6. Look up missing fruit `"Dragonfruit"` with `prices.get("Dragonfruit")` using `match`:
   `"Sorry, Dragonfruit is not in stock!"`
7. Insert `"Orange"` at `1.90` using the entry API:
   ```rust
   prices.entry("Orange").or_insert(1.90);
   ```
8. Print the total number of fruits in catalog using `prices.len()`.

---

## 📋 Expected Output on Your Screen
```text
=== FRESH MARKET PRICE CATALOG ===
Initial catalog items: 3
Has 'Banana'? true
Lookup [Mango]: Price of Mango: $3.75
Lookup [Dragonfruit]: Sorry, Dragonfruit is not in stock!
Adding Orange via entry API...
Updated catalog count: 4
==================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_40_hashmaps/src/main.rs).
2. Follow the TODOs to insert, query with `.get()`, and add with `.entry().or_insert()`.
3. Test:
   ```bash
   cd tasks/04_collections/task_40_hashmaps
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_40_hashmaps/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_40_hashmaps/solution/explain.md)

