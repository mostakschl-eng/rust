# Task 17: Dungeon Treasure Room (Scope & Code Blocks)

## 🎯 What is Your Task?
Write a Rust program simulating a hero entering a secret **Dungeon Treasure Room** using nested scopes `{ }`.

Requirements:
1. In `main()`, declare:
   `let gold = 100;` (Hero starts with 100 gold)
2. Print `"Outside dungeon: Gold = 100"`
3. Create an inner scope block `{ ... }`:
   - Inside the room, declare `let dungeon_chest = 500;`
   - Declare a temporary shadowed `let gold = gold + dungeon_chest;`
   - Print `"Inside secret room: Gold = 600"`
4. Outside the block, print `"Back outside room: Gold = 100"`
5. Observe that the inner name is out of scope after the block. You can temporarily try printing dungeon_chest outside, read the compiler error, then remove that extra line so the finished task compiles.

---

## 📋 Expected Output on Your Screen
```text
=== DUNGEON ADVENTURE ===
Outside dungeon:   Gold = 100
[Entered Secret Vault]
Inside secret room: Gold = 600
[Exited Secret Vault]
Back outside room: Gold = 100
=========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Create the inner scope block with `{ ... }`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_17_scope_blocks
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Keep the starting gold in the outer block and the chest in an inner block.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Use a new let gold inside the inner block. After it closes, the outer gold name becomes visible again and still refers to 100.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

