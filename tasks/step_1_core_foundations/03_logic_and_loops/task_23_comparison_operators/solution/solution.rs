// Official Solution for Task 23: Digital Security Vault PIN Validator (Comparison Operators)

fn main() {
    let correct_pin = "9876";
    let entered_pin = "9876";

    let attempt_count = 1;
    let max_attempts = 3;

    let is_pin_match = entered_pin == correct_pin;
    let within_limit = attempt_count <= max_attempts;
    let not_at_limit = attempt_count != max_attempts;

    println!("=== DIGITAL SECURITY VAULT ===");
    println!("Checking PIN:          Validating...");
    println!("PIN Match Result:      {}", is_pin_match);
    println!("Within Attempt Limit:  {}", within_limit);
    println!("Not At Attempt Limit:  {}", not_at_limit);

    if is_pin_match {
        println!("Vault Access Granted! Door opening... 🔓");
    }

    println!("==============================");
}

