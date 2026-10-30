// Task 20: The Compiler Doctor (Fixing 3 Intentional Bugs)
//
// INSTRUCTIONS:
// Fix the 3 bugs below:
// Bug 1: Needs `mut` on prescription_count
// Bug 2: Needs `as f64` before multiplying with price_per_pill
// Bug 3: Needs `{:?}` instead of `{}` to print doctor_badge
// Test by running: cargo run

fn main() {
    println!("=== COMPILER DOCTOR: PATIENT CURED ===");

    // TODO: BUG 1: Fix mutability here:
    let prescription_count = 1;
    println!("Initial Prescription: {}", prescription_count);
    prescription_count = 3;

    let price_per_pill: f64 = 1.50;

    // TODO: BUG 2: Fix type multiplication with `as f64`:
    let total_cost = prescription_count * price_per_pill;

    let doctor_badge = ("Dr. Rahim", 101);

    println!("Prescription Count: {}", prescription_count);
    println!("Price Per Pill:     ${:.2}", price_per_pill);
    println!("Total Medicine:     ${:.2}", total_cost);

    // TODO: BUG 3: Fix tuple printing with `{:?}`:
    println!("Doctor Credentials: {}", doctor_badge);
    println!("Status:             Healthy & Compiling!");
    println!("======================================");
}
