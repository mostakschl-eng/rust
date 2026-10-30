use std::num::ParseIntError;

// TODO 1: Implement calculate_volume with ? operator
fn calculate_volume(w_str: &str, h_str: &str, d_str: &str) -> Result<i32, ParseIntError> {
    let w: i32 = w_str.parse()?;
    let h: i32 = h_str.parse()?;
    let d: i32 = d_str.parse()?;

    Ok(w * h * d)
}

fn main() {
    println!("=== CARGO VOLUME CALCULATOR ===");

    println!("Batch 1 (Valid Input):");
    match calculate_volume("10", "20", "5") {
        Ok(vol) => println!("Result: Ok({} cm³)", vol),
        Err(e) => println!("Result: Err({:?})", e),
    }

    println!("\nBatch 2 (Malformed Input):");
    match calculate_volume("10", "invalid_height", "5") {
        Ok(vol) => println!("Result: Ok({} cm³)", vol),
        Err(e) => println!("Result: Err({:?})", e),
    }

    println!("===============================");
}

