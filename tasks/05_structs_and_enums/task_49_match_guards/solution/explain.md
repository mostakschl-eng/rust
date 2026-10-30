# Solution Explanation: Task 49 (Match Guards)

Let's dissect how Match Guards combine pattern matching with dynamic runtime conditions!

---

## 🔍 The Solution Code

```rust
#[allow(dead_code)]
enum QuestReward {
    XP(u32),
    Gold(u32),
    Item(String),
}

fn evaluate_reward(reward: QuestReward, is_double_xp: bool) {
    match reward {
        QuestReward::XP(xp) if is_double_xp => {
            println!("⚡ Double XP Event Active! Awarded {} XP!", xp * 2);
        }
        QuestReward::XP(xp) if xp >= 1000 => {
            println!("🏆 Epic Quest Milestone! Awarded {} XP!", xp);
        }
        QuestReward::XP(xp) => {
            println!("⭐ Standard Quest: Awarded {} XP.", xp);
        }
        QuestReward::Gold(g) => {
            println!("🪙 Pouch Collected: Earned {} gold coins!", g);
        }
        QuestReward::Item(item) => {
            println!("⚔️ Loot Acquired: {}!", item);
        }
    }
}
```

---

## 🧩 Key Takeaways

1. **Guard Evaluation Order:**
   * Rust checks match arms top to bottom.
   * If `is_double_xp` is true, the first arm matches and doubles `xp`.
   * If false, it falls through to check `xp >= 1000`.
   * If both guards fail, it hits the unguarded `QuestReward::XP(xp)` arm!
2. **Exhaustiveness Guarantee:**
   * Because match guards might evaluate to false at runtime, Rust requires an unguarded pattern arm to ensure all cases are handled.

