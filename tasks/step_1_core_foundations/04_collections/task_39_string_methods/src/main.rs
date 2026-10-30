fn main() {
    println!("=== SECURITY BROADCAST SANITIZER ===");

    let raw_input = "   ALERT: critical system warning!   ";
    println!("Raw Length:     {} bytes", raw_input.len());

    // TODO 1: Trim leading and trailing spaces
    let clean = raw_input.trim();
    println!("Trimmed:        '{}'", clean);
    println!("Trimmed Length: {} bytes", clean.len());

    // TODO 2: Check if message contains "critical"
    let is_critical = clean.contains("critical");
    println!("Has 'critical'? {}", is_critical);

    // TODO 3: Replace "warning" with "EMERGENCY 🚨"
    let replaced = clean.replace("warning", "EMERGENCY 🚨");
    println!("Replaced:       '{}'", replaced);

    // TODO 4: Format final uppercase announcement using format! macro
    let upper = clean.to_uppercase();
    let final_notice = format!("BROADCAST: {}", upper);
    println!("Final Notice:   {}", final_notice);

    println!("====================================");
}

