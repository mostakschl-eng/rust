// Task 28: Security PIN Brute-Force Lockout (`while` Loop)
//
// INSTRUCTIONS:
// 1. Declare `let mut attempts_used = 0;` and `let max_allowed = 3;`
// 2. Loop while `attempts_used < max_allowed`.
// 3. Increment attempts and print error message.
// 4. Print lockout alert after loop finishes.
// 5. Test with `cargo run`.

fn main() {
    println!("SECURITY SYSTEM: Monitoring login attempts...");
    let mut attempts_used = 0;
    let max_allowed = 3;

    // TODO: Write your while loop here:
    while attempts_used < max_allowed {
        attempts_used += 1;
        println!("Attempt: {}", attempts_used);
        break;
    }

    println!("SECURITY ALERT: Maximum attempts reached! Account Locked! 🔒");
}
