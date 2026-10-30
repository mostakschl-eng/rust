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

fn main() {
    println!("=== RPG QUEST REWARD ENGINE ===");

    evaluate_reward(QuestReward::XP(500), true);
    evaluate_reward(QuestReward::XP(1200), false);
    evaluate_reward(QuestReward::XP(300), false);
    evaluate_reward(QuestReward::Gold(250), false);
    evaluate_reward(QuestReward::Item(String::from("Excalibur")), false);

    println!("===============================");
}

