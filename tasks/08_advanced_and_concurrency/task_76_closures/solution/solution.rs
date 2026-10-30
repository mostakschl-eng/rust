fn main() {
    println!("=== BILL CALCULATOR WITH CLOSURES ===");

    let original_price = 150.00;
    let discount_rate = 0.10; // 10%
    let tax_rate = 0.08;      // 8%

    let apply_discount = |price: f64| price * (1.0 - discount_rate);
    let apply_tax = |price: f64| price * (1.0 + tax_rate);

    let format_bill = |raw_price: f64| {
        let discounted = apply_discount(raw_price);
        apply_tax(discounted)
    };

    let discounted_price = apply_discount(original_price);
    let final_bill = format_bill(original_price);

    println!("Original Price:  ${:.2}", original_price);
    println!("Discounted:      ${:.2}", discounted_price);
    println!("Final Bill:      ${:.2}", final_bill);

    println!("=====================================");
}

