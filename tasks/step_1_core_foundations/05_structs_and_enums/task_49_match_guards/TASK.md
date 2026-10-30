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

1. Open [`src/main.rs`](src/main.rs).
2. Use `Pattern if condition => { ... }` inside the `match` expression.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_49_match_guards
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Check the double-XP event first, then the large-XP milestone.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Include an unguarded XP fallback after the guarded XP arms. Widen xp to u64 before doubling so even a maximum u32 reward fits.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Only one arm executes. During double XP, a 1200-point reward follows the first arm, not the milestone arm.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
