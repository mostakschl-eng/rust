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

1. Open [`src/main.rs`](src/main.rs).
2. Access index 1 and 3 using `codes[1]` and `codes[3]`.
3. Use `match codes.get(2)` and `match codes.get(9)`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_34_array_indexing
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Use direct indexing only for the known-valid positions 1 and 3.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

For each `.get(...)`, write a `match` with both `Some(code)` and `None` arms.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

An empty collection has no last index. `.get()` handles invalid indices without indexing; it does not promise the whole application cannot crash.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.

## Small prerequisite bridge: optional results and matching

Think of `Option<T>` as a parcel that either contains a value (`Some(value)`) or is empty (`None`). Here `.get(i)` returns `Option<&T>`: it borrows an element instead of copying or moving it. `Some(value)` is a **pattern** that gives the borrowed element the local name `value`; it is not a new lookup. `match` chooses one arm and must handle both possibilities. `=>` separates a pattern from its action. Lessons 46–50 develop enums and matching fully.

