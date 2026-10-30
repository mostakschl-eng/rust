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

1. Open [`src/main.rs`](src/main.rs).
2. Follow the TODOs to insert, query with `.get()`, and add with `.entry().or_insert()`.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_40_hashmaps
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Import HashMap and insert the three initial pairs before querying them.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

String literals already have type `&str`; use `.get("Mango")`, handle `Some(price)` and `None`, then insert Orange through entry.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

HashMap iteration order is unspecified. This task prints explicit lookups and counts so its output is stable.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
