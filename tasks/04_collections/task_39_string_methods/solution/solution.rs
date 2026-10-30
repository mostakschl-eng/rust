fn main() {
    println!("=== SECURITY BROADCAST SANITIZER ===");

    let raw_input = "   ALERT: critical system warning!   ";
    println!("Raw Length:     {} bytes", raw_input.len());

    let clean = raw_input.trim();
    println!("Trimmed:        '{}'", clean);
    println!("Trimmed Length: {} bytes", clean.len());

    let is_critical = clean.contains("critical");
    println!("Has 'critical'? {}", is_critical);

    let replaced = clean.replace("warning", "EMERGENCY 🚨");
    println!("Replaced:       '{}'", replaced);

    let upper = clean.to_uppercase();
    let final_notice = format!("BROADCAST: {}", upper);
    println!("Final Notice:   {}", final_notice);

    println!("====================================");
}

