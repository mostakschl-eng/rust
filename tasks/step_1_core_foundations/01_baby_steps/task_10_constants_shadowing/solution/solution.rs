// Official Solution for Task 10: Store Pricing & Tax Calculator (Constants & Shadowing)

const TAX_PERCENTAGE: i32 = 10;

fn main() {
    println!("=== SUPERSTORE CHECKOUT ===");
    println!("Tax Rate:             {}%", TAX_PERCENTAGE);

    let price = 100;
    println!("Original Price:       ${}", price);

    // Shadowing step 1: Apply $20 discount
    let price = price - 20;
    println!("Price After Coupon:   ${}", price);

    // Shadowing step 2: Add 10% tax ($80 + $8 = $88)
    let tax_amount = price * TAX_PERCENTAGE / 100;
    let price = price + tax_amount;
    println!("Final Price With Tax: ${}", price);

    println!("===========================");
}

