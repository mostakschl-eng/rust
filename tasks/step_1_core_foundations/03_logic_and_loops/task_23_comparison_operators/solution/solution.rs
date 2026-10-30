// Official Solution for Task 23: Digital Security Vault PIN Validator (Comparison Operators)

fn main() {
    let correct_pin = "9876";
    let entered_pin = "9876";

    let attempt_count = 1;
    let max_attempts = 3;

    let is_pin_match = entered_pin == correct_pin;
    let attempts_remaining = attempt_count <= max_attempts;

    println!("=== DIGITAL SECURITY VAULT ===");
    println!("Checking PIN:          Validating...");
    println!("PIN Match Result:      {}", is_pin_match);
    println!("Attempts Remaining:    {}", attempts_remaining);

    if is_pin_match {
        println!("Vault Access Granted! Door opening... 🔓");
    }

    println!("==============================");
}

