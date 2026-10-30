fn calculate_stats(s1: i32, s2: i32, s3: i32) -> (i32, f64, i32) {
    let sum = s1 + s2 + s3;
    let avg = sum as f64 / 3.0;
    let mut max = s1;
    if s2 > max {
        max = s2;
    }
    if s3 > max {
        max = s3;
    }
    (sum, avg, max)
}

fn main() {
    println!("=== EXAM BATCH 1 RESULTS ===");

    let (total, average, highest) = calculate_stats(85, 92, 78);

    println!("Total Points:  {}", total);
    println!("Average Score: {:.2}", average);
    println!("Highest Score: {}", highest);

    println!("\n=== EXAM BATCH 2 RESULTS ===");

    let (_, _, top_score) = calculate_stats(90, 95, 100);

    println!("Batch 2 Top Score: {}", top_score);
    println!("============================");
}

