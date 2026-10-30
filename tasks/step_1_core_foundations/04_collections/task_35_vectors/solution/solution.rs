fn main() {
    println!("=== ARCADE LEADERBOARD ===");

    let mut high_scores: Vec<i32> = Vec::new();
    high_scores.push(1250);
    high_scores.push(3400);
    high_scores.push(2950);

    println!("Total scores recorded: {}", high_scores.len());
    println!("First recorded score: {} pts", high_scores[0]);
    println!("All scores: {:?}", high_scores);

    println!("\n=== PROGRAMMING LANGUAGES ===");

    let languages = vec!["Rust", "Python", "TypeScript"];

    println!("Favorite language: {}", languages[0]);
    println!("All languages: {:?}", languages);

    println!("=============================");
}

