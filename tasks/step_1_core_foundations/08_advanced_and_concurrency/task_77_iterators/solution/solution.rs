fn main() {
    println!("=== GAMER SCORE PIPELINE ===");

    let scores = vec![45, 92, 12, 85, 33, 76, 58, 99, 64, 88];

    let honors_scores: Vec<i32> = scores
        .iter()
        .filter(|&&s| s >= 70)
        .map(|&s| s + 5)
        .collect();

    let total_honors: i32 = honors_scores.iter().sum();

    println!("Original Scores: {:?}", scores);
    println!("Honors (+5):     {:?}", honors_scores);
    println!("Total Honors:    {}", total_honors);

    println!("============================");
}

