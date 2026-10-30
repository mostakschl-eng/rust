# Task 36: Shopping Cart Operations (Push, Pop, Clear)

## 🎯 What is Your Task?
Write a Rust program to manage a digital shopping cart:
1. Create a mutable vector `cart = Vec::new()`.
2. Push three items: `"Apples"`, `"Bread"`, `"Milk"`.
3. Print the number of items using `.len()`.
4. Pop the last item using `.pop()`, and print what was removed.
5. Print the remaining items in the cart using `{:?}`.
6. Empty the cart using `.clear()`.
7. Check if the cart is empty using `.is_empty()`, and print `"Cart successfully cleared!"`.

---

## 📋 Expected Output on Your Screen
```text
=== GROCERY CART MANAGER ===
Added items! Cart count: 3
Popped item from cart: Milk
Remaining items: ["Apples", "Bread"]
Clearing all items...
Cart is empty? true
Cart successfully cleared! 🛒
============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Call `cart.push(...)`.
3. Use `match cart.pop() { Some(item) => ..., None => ... }`.
4. Call `cart.clear()` and check `cart.is_empty()`.
5. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_36_vector_operations
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Push the items in order; popping removes Milk.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Handle both `Some(item)` and `None`, then clear and check `is_empty()`.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

`pop()` removes an owned value; `get()` only borrows. Clearing does not necessarily release the allocation.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
