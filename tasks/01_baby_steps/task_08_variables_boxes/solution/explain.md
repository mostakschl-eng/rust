# Task 08: Solution Breakdown & Variable Explanations

## 1. Line-by-Line Breakdown
* `let hero_name = "Warrior Rahim";`: Rust creates a text box in memory.
* `let hero_level = 5;`: Rust creates an integer box.
* `let potion_cost = 50;`: Instead of hardcoding `50` in multiple places, putting numbers in named variables makes your code readable and self-documenting!
* `let remaining_gold = hero_gold - potion_cost;`: The computer looks inside `hero_gold` (250) and `potion_cost` (50), subtracts them (200), and places the answer in `remaining_gold`.

---

## 2. Immutability by Default
Notice that we did not change `hero_gold` itself—we created a new variable `remaining_gold`. In the next lesson, we will learn how to actually update existing variable boxes in-place using `mut`!
