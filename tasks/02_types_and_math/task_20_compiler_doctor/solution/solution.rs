// Official Solution for Task 20: The Compiler Doctor (Fixing 3 Intentional Bugs)

fn main() {
    println!("=== COMPILER DOCTOR: PATIENT CURED ===");

    // Cure 1: Added `mut`
    let mut prescription_count = 1;
    println!("Initial Prescription: {}", prescription_count);
    prescription_count = 3;

    let price_per_pill: f64 = 1.50;

    // Cure 2: Cast integer to float with `as f64`
    let total_cost = (prescription_count as f64) * price_per_pill;

    let doctor_badge = ("Dr. Rahim", 101);

    println!("Prescription Count: {}", prescription_count);
    println!("Price Per Pill:     ${:.2}", price_per_pill);
    println!("Total Medicine:     ${:.2}", total_cost);

    // Cure 3: Used debug printing `{:?}` for the tuple
    println!("Doctor Credentials: {:?}", doctor_badge);
    println!("Status:             Healthy & Compiling!");
    println!("======================================");
}
