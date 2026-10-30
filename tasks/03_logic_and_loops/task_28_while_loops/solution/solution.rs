// Official Solution for Task 28: Security PIN Brute-Force Lockout (`while` Loop)

fn main() {
    println!("SECURITY SYSTEM: Monitoring login attempts...");
    let mut attempts_used = 0;
    let max_allowed = 3;

    while attempts_used < max_allowed {
        attempts_used += 1;
        println!("Attempt {} of {}: Incorrect PIN! ❌", attempts_used, max_allowed);
    }

    println!("SECURITY ALERT: Maximum attempts exceeded! Account Locked! 🔒");
}

