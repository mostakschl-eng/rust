// TODO 1: Implement sum_slice(data: &[i32]) -> i32
fn sum_slice(data: &[i32]) -> i32 {
    let mut total = 0;
    for &num in data {
        total += num;
    }
    total
}

fn main() {
    println!("=== TELEMETRY SLICE ANALYZER ===");

    println!("[1. Text Slices]");
    let sentence = String::from("Rustacean Power");

    // TODO 2: Slice word1 and word2
    let word1 = &sentence[0..9];
    let word2 = &sentence[10..];

    println!("Word 1: '{}'", word1);
    println!("Word 2: '{}'", word2);

    println!("\n[2. Numeric Array Slices]");
    let readings = [12, 45, 67, 89, 90, 34, 21];

    // TODO 3: Extract peak slice [2..5] and sum
    let peak = &readings[2..5];

    println!("Full Readings: {:?} (Count: {})", readings, readings.len());
    println!("Peak Window:   {:?} (Count: {})", peak, peak.len());
    println!("Peak Energy:   {} units", sum_slice(peak));

    println!("================================");
}

