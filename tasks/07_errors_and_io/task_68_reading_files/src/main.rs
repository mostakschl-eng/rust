use std::fs;

// TODO 1: Implement count_file_stats(text: &str) -> (usize, usize)
fn count_file_stats(text: &str) -> (usize, usize) {
    (text.lines().count(), text.split_whitespace().count())
}

fn main() {
    println!("=== MISSION LOG FILE READER ===");

    println!("[1. Reading Existing Mission Log]");
    let sample_file = "sample_mission.txt";
    let sample_text = "Mission: Alpha\nTarget: Mars Base 1\nStatus: Active\n";
    let _ = fs::write(sample_file, sample_text);

    // TODO 2: Read sample_file with fs::read_to_string
    match fs::read_to_string(sample_file) {
        Ok(contents) => {
            println!("File Contents:\n{}", contents.trim());
            let (lines, words) = count_file_stats(&contents);
            println!("\nLine Count: {}", lines);
            println!("Word Count: {}", words);
        }
        Err(e) => println!("Error reading file: {}", e),
    }

    println!("\n[2. Reading Non-Existent File]");
    // TODO 3: Read "non_existent.txt" safely
    match fs::read_to_string("non_existent.txt") {
        Ok(_) => println!("Found file!"),
        Err(e) => println!("⚠️ Handled Gracefully: {}", e),
    }

    println!("===============================");
}

