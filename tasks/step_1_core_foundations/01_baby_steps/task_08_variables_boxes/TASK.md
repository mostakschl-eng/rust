# Task 08: RPG Game Character Profile (Variables)

## 🎯 What is Your Task?
Write a Rust program that stores a video game character's attributes in **variables** using `let` and prints a formatted character sheet.

You must create 4 variables:
1. `hero_name`: Text (e.g. `"Warrior Rahim"`)
2. `hero_level`: Whole number (e.g. `5`)
3. `hero_health`: Whole number (e.g. `100`)
4. `hero_gold`: Whole number (e.g. `250`)

Calculate one extra variable:
5. `remaining_gold`: Subtract `50` gold from `hero_gold` to buy a potion!

---

## 📋 Expected Output on Your Screen
```text
=== CHARACTER PROFILE ===
Hero Name:         Warrior Rahim
Current Level:     5
Health Points:     100 HP
Starting Gold:     250 coins
Bought healing potion for 50 coins!
Remaining Gold:    200 coins
=========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare each variable using `let`.
3. Perform the subtraction: `let remaining_gold = hero_gold - 50;`.
4. Print all values using `println!`.
5. Test with:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_08_variables_boxes
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Create named bindings for the four starting attributes before printing them.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Create a new remaining_gold binding from starting gold minus the potion cost; printing the new value does not change the original.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

