# Task 31: The Cargo Shipment Label (Tuples)

## 🎯 What is Your Task?
Write a Rust program that creates a shipping parcel tuple bundling:
1. `tracking_id`: integer `10482`
2. `destination`: string `"Tokyo"`
3. `weight_kg`: decimal `4.75`
4. `is_delivered`: boolean `false`

Print each piece of information using dot-indexing (`.0`, `.1`, `.2`, `.3`), and print the entire raw tuple using debug formatting `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== AIR CARGO SHIPMENT LABEL ===
Tracking ID:  #10482
Destination:  Tokyo
Weight:       4.75 kg
Delivered?    false
Raw Parcel:   (10482, "Tokyo", 4.75, false)
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Create the tuple:
   ```rust
   let parcel = (10482, "Tokyo", 4.75, false);
   ```
3. Print each field with `.0`, `.1`, `.2`, `.3`.
4. Print the entire tuple with `{:?}`.
5. Test your code:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_31_tuples
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Start by identifying which parcel value occupies each position.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Use `.0` for the ID and `{:?}` to print the entire tuple.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Tuple field indices are literal numbers, not runtime variables.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
