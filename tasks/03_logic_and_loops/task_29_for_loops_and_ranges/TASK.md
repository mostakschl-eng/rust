# Task 29: Push-Up Repetition Counter (`for` Loops & Ranges)

## 🎯 What is Your Task?
Write a Rust program simulating an **Automated Gym Fitness Coach** counting push-up repetitions from 1 to 10 using a `for` loop and inclusive range `1..=10`.

Rules:
1. Print `"COACH: Set 1 — Starting 10 push-ups! Let's go! 💪"`
2. Loop through `1..=10`:
   - Print `"Rep {}: Good form!"`
   - If `rep == 5`: Print `"Halfway there! Keep pushing! 🔥"`
3. After the loop completes, print `"COACH: Set Complete! Great work! Take a 30-second rest. 🏆"`

---

## 📋 Expected Output on Your Screen
```text
COACH: Set 1 — Starting 10 push-ups! Let's go! 💪
Rep 1: Good form!
Rep 2: Good form!
Rep 3: Good form!
Rep 4: Good form!
Rep 5: Good form!
Halfway there! Keep pushing! 🔥
Rep 6: Good form!
Rep 7: Good form!
Rep 8: Good form!
Rep 9: Good form!
Rep 10: Good form!
COACH: Set Complete! Great work! Take a 30-second rest. 🏆
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_29_for_loops_and_ranges/src/main.rs).
2. Write the `for rep in 1..=10 { ... }` loop.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_29_for_loops_and_ranges
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: For loop with condition</b></summary>

```rust
for rep in 1..=10 {
    println!("Rep {}: Good form!", rep);
    if rep == 5 {
        println!("Halfway there! Keep pushing! 🔥");
    }
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_29_for_loops_and_ranges/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_29_for_loops_and_ranges/solution/explain.md)

