// TODO 1: Implement split_bill(total_amount: f64, people_count: u32) -> Result<f64, String>
fn split_bill(total_amount: f64, people_count: u32) -> Result<f64, String> {
    if people_count == 0 {
        Err(String::from("Cannot divide bill among 0 diners!"))
    } else {
        Ok(total_amount / people_count as f64)
    }
}

fn main() {
    println!("=== RESTAURANT BILL SPLITTER ===");

    println!("Attempt 1 (Valid Group):");
    match split_bill(120.0, 4) {
        Ok(share) => println!("Bill split successfully: ${:.2} per person", share),
        Err(err) => println!("⚠️ Calculation Rejected: {}", err),
    }

    println!("\nAttempt 2 (Zero Group):");
    match split_bill(50.0, 0) {
        Ok(share) => println!("Bill split successfully: ${:.2} per person", share),
        Err(err) => println!("⚠️ Calculation Rejected: {}", err),
    }

    println!("\nAttempt 3 (Fallback Unwrapping):");
    let fallback_share = split_bill(80.0, 0).unwrap_or(0.0);
    println!("Fallback calculation: ${:.2}", fallback_share);

    println!("================================");
}

