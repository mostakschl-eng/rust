# Task 49: Quest Reward Evaluation Engine (Match Guards)

## 🎯 What is Your Task?
Write a Rust program to evaluate RPG quest rewards with conditional match guards:
1. Define an enum `QuestReward`:
   - `XP(u32)`
   - `Gold(u32)`
   - `Item(String)`
2. Write a function `evaluate_reward(reward: QuestReward, is_double_xp: bool)`:
   - Match `QuestReward::XP(xp)` with guard `if is_double_xp` -> `"⚡ Double XP Event Active! Awarded <xp * 2> XP!"`
   - Match `QuestReward::XP(xp)` with guard `if xp >= 1000` -> `"🏆 Epic Quest Milestone! Awarded <xp> XP!"`
   - Match `QuestReward::XP(xp)` fallback -> `"⭐ Standard Quest: Awarded <xp> XP."`
   - Match `QuestReward::Gold(g)` -> `"🪙 Pouch Collected: Earned <g> gold coins!"`
   - Match `QuestReward::Item(item)` -> `"⚔️ Loot Acquired: <item>!"`
3. Test in `main()` with:
   - `QuestReward::XP(500)` during Double XP.
   - `QuestReward::XP(1200)` without Double XP.
   - `QuestReward::XP(300)` without Double XP.
   - `QuestReward::Gold(250)`.
   - `QuestReward::Item("Excalibur")`.

---

## 📋 Expected Output on Your Screen
```text
=== RPG QUEST REWARD ENGINE ===
⚡ Double XP Event Active! Awarded 1000 XP!
🏆 Epic Quest Milestone! Awarded 1200 XP!
⭐ Standard Quest: Awarded 300 XP.
🪙 Pouch Collected: Earned 250 gold coins!
⚔️ Loot Acquired: Excalibur!
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_49_match_guards/src/main.rs).
2. Use `Pattern if condition => { ... }` inside the `match` expression.
3. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_49_match_guards
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_49_match_guards/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_49_match_guards/solution/explain.md)

