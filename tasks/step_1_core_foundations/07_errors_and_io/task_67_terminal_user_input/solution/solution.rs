use std::io;

fn parse_terminal_input(raw: &str) -> (String, u32) {
    let clean = raw.trim();
    let mut parts = clean.split_whitespace();
    let name = parts.next().unwrap_or("Guest").to_string();
    let age = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(18);
    (name, age)
}

fn main() -> io::Result<()> {
    assert_eq!(parse_terminal_input("  Tanvir 22\r\n"), ("Tanvir".to_string(), 22));
    assert_eq!(parse_terminal_input(""), ("Guest".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya invalid"), ("Maya".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya -1"), ("Maya".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya 4294967296"), ("Maya".to_string(), 18));

    println!("=== TERMINAL INPUT SANITIZER ===");
    println!("Enter a name and age (for example: Tanvir 22):");
    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input)?;
    let (name, age) = parse_terminal_input(&raw_input);
    println!("Sanitized Name:  {}", name);
    println!("Parsed Age:      {} years old", age);
    println!("================================");
    Ok(())
}
