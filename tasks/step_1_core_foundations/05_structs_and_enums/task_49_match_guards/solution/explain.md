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
            println!("⚡ Double XP Event Active! Awarded {} XP!", u64::from(xp) * 2);
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

## Arithmetic limit

`xp` is `u32`. `u64::from(xp)` converts it exactly to a larger unsigned integer **before** multiplication, so doubling even `u32::MAX` fits. The enum and normal example outputs remain unchanged. When the double-XP flag is true, that first arm wins even for a reward above 1000.

## Everyday picture and symbols

Think of a reward desk checking an event rule before an ordinary reward rule. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `Pattern if condition =>` | Match arm with an extra condition |
| `u64::from(xp)` | Widen a u32 before arithmetic |
| `_` | Wildcard covering remaining values |

## Common pitfall

Only one arm executes. During double XP, a 1200-point reward follows the first arm, not the milestone arm.

## Expected output

```text
=== RPG QUEST REWARD ENGINE ===
⚡ Double XP Event Active! Awarded 1000 XP!
🏆 Epic Quest Milestone! Awarded 1200 XP!
⭐ Standard Quest: Awarded 300 XP.
🪙 Pouch Collected: Earned 250 gold coins!
⚔️ Loot Acquired: Excalibur!
===============================
```
