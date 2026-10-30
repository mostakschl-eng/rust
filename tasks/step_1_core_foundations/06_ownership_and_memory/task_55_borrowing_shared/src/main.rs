// TODO 1: Implement analyze_text(text: &String) -> (usize, usize)
fn analyze_text(text: &String) -> (usize, usize) {
    (text.len(), text.split_whitespace().count())
}

fn main() {
    println!("=== SHARED BORROWING TEXT ANALYZER ===");

    let manifesto = String::from("Rust empowers everyone to build reliable and efficient software.");

    // TODO 2: Borrow with &manifesto
    let (bytes, words) = analyze_text(&manifesto);

    println!("Total Bytes: {}", bytes);
    println!("Total Words: {}", words);
    println!("Original Text (Still Alive!):\n\"{}\"", manifesto);

    println!("\nMultiple Simultaneous Readers:");
    // TODO 3: Create r1 and r2 borrowing &manifesto
    let r1 = &manifesto;
    let r2 = &manifesto;

    println!("Reader 1: {}", r1);
    println!("Reader 2: {}", r2);

    println!("======================================");
}

