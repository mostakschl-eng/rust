# Task 34: Server Status Code Vault & Safe Lookup

## 🎯 What is Your Task?
Write a Rust program to look up HTTP server status codes stored in a fixed array:
1. Declare `let codes = [101, 204, 301, 404, 500];`.
2. Print direct access:
   - Index 1 (Code 204)
   - Index 3 (Code 404)
3. Use the safe `.get()` method with `match` to check index 2:
   - Print `"Found status code: 301"`
4. Use `.get()` with `match` to check out-of-bounds index 9:
   - Print `"⚠️ Status code at index 9 does not exist! (Safely handled, zero crashes)"`

---

## 📋 Expected Output on Your Screen
```text
=== SERVER STATUS CODE LOOKUP ===
Direct check [Index 1]: 204
Direct check [Index 3]: 404
Lookup [Index 2]: Found status code: 301
Lookup [Index 9]: ⚠️ Status code at index 9 does not exist! (Safely handled, zero crashes)
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_34_array_indexing/src/main.rs).
2. Access index 1 and 3 using `codes[1]` and `codes[3]`.
3. Use `match codes.get(2)` and `match codes.get(9)`.
4. Test:
   ```bash
   cd tasks/04_collections/task_34_array_indexing
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_34_array_indexing/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_34_array_indexing/solution/explain.md)

