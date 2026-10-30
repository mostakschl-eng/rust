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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_31_tuples/src/main.rs).
2. Create the tuple:
   ```rust
   let parcel = (10482, "Tokyo", 4.75, false);
   ```
3. Print each field with `.0`, `.1`, `.2`, `.3`.
4. Print the entire tuple with `{:?}`.
5. Test your code:
   ```bash
   cd tasks/04_collections/task_31_tuples
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Tuple Indexing</b></summary>

```rust
println!("Tracking ID:  #{}", parcel.0);
println!("Destination:  {}", parcel.1);
println!("Weight:       {} kg", parcel.2);
println!("Delivered?    {}", parcel.3);
println!("Raw Parcel:   {:?}", parcel);
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_31_tuples/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_31_tuples/solution/explain.md)

