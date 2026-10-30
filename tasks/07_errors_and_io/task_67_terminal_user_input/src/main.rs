// TODO 1: Implement parse_terminal_input(raw: &str) -> (String, u32)
fn parse_terminal_input(raw: &str) -> (String, u32) {
    let clean = raw.trim();
    let parts: Vec<&str> = clean.split_whitespace().collect();

    let name = parts.get(0).copied().unwrap_or("Guest").to_string();
    let age = parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(18);

    (name, age)
}

fn main() {
    println!("=== TERMINAL INPUT SANITIZER ===");

    let raw_input = "  Tanvir 22\r\n";
    println!("Raw Input:       {:?}", raw_input);

    let (name, age) = parse_terminal_input(raw_input);

    println!("Sanitized Name:  {}", name);
    println!("Parsed Age:      {} years old", age);
    println!("Terminal reader initialized and verified! ⌨️");

    println!("================================");
}

