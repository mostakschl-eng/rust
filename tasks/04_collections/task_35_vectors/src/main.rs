fn main() {
    println!("=== ARCADE LEADERBOARD ===");

    // TODO 1: Create mutable high_scores vector with Vec::new()
    let mut high_scores: Vec<i32> = Vec::new();

    // TODO 2: Push scores 1250, 3400, 2950
    high_scores.push(1250);
    high_scores.push(3400);
    high_scores.push(2950);

    println!("Total scores recorded: {}", high_scores.len());
    println!("Top score: {} pts", high_scores[0]);
    println!("All scores: {:?}", high_scores);

    println!("\n=== PROGRAMMING LANGUAGES ===");

    // TODO 3: Create languages vector with vec!["Rust", "Python", "TypeScript"]
    let languages = vec!["Rust", "Python", "TypeScript"];

    println!("Favorite language: {}", languages[0]);
    println!("All languages: {:?}", languages);

    println!("=============================");
}

