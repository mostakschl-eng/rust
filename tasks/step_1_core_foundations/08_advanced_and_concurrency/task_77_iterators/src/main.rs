fn main() {
    println!("=== GAMER SCORE PIPELINE ===");

    let scores = vec![45, 92, 12, 85, 33, 76, 58, 99, 64, 88];

    // TODO 1: Build iterator pipeline:
    // 1. .iter() to borrow elements
    // 2. .filter() to keep score >= 70
    // 3. .map() to add 5 bonus points
    // 4. .collect() into Vec<i32>
    let honors_scores: Vec<i32> = scores
        .iter()
        .filter(|&&s| s >= 70)
        .map(|&s| s + 5)
        .collect();

    // TODO 2: Calculate sum of honors_scores using .iter().sum()
    let total_honors: i32 = honors_scores.iter().sum();

    println!("Original Scores: {:?}", scores);
    println!("Honors (+5):     {:?}", honors_scores);
    println!("Total Honors:    {}", total_honors);

    println!("============================");
}

