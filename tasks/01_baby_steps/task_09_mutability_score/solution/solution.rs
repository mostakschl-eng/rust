// Official Solution for Task 09: Arcade Score Tracker (Mutability with `mut`)

fn main() {
    println!("=== ARCADE LEADERBOARD ===");
    let mut score = 0;
    println!("Game Started! Starting Score: {}", score);

    score = score + 150;
    println!("Round 1: Defeated monster (+150) -> Score: {}", score);

    score = score + 300;
    println!("Round 2: Secret treasure  (+300) -> Score: {}", score);

    score = score - 50;
    println!("Round 3: Trap penalty     (-50)  -> Score: {}", score);

    println!("=== FINAL SCORE: {} POINTS ===", score);
}

