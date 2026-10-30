// TODO 1: Implement print_badge function taking two &str parameters
fn print_badge(role: &str, code: &str) {
    println!("[BADGE] Role: {} | Code: {}", role, code);
}

fn main() {
    println!("=== BADGE DISPATCH SYSTEM ===");

    // TODO 2: Declare role_slice as &str
    let role_slice: &str = "SECURITY CHIEF";

    // TODO 3: Create mutable badge_code String from "SEC"
    let mut badge_code = String::from("SEC");

    // TODO 4: Append '-' and "9042"
    badge_code.push('-');
    badge_code.push_str("9042");

    // TODO 5: Call print_badge passing role_slice and &badge_code
    print_badge(role_slice, &badge_code);

    println!("=============================");
}

