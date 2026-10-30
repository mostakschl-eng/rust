fn print_badge(role: &str, code: &str) {
    println!("[BADGE] Role: {} | Code: {}", role, code);
}

fn main() {
    println!("=== BADGE DISPATCH SYSTEM ===");

    let role_slice: &str = "SECURITY CHIEF";

    let mut badge_code = String::from("SEC");
    badge_code.push('-');
    badge_code.push_str("9042");

    print_badge(role_slice, &badge_code);

    println!("=============================");
}

