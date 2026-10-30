fn main() {
    println!("=== INVOICE & DISCOUNT ENGINE ===");

    let mut prices = vec![12.0, 45.5, 9.99, 120.0];

    // TODO 1: Loop over &prices to calculate original sum
    let mut original_sum = 0.0;
    for price in &prices {
        original_sum += price;
    }
    println!("Original Total: ${:.2}", original_sum);

    println!("\nApplying 10% holiday discount to all items...");
    // TODO 2: Loop over &mut prices and discount each by multiplying by 0.90
    for price in &mut prices {
        *price *= 0.90;
    }

    println!("=== DISCOUNTED RECEIPT ===");
    // TODO 3: Loop over prices.iter().enumerate() and print numbered lines
    for (idx, price) in prices.iter().enumerate() {
        println!("Item #{}: ${:.2}", idx + 1, price);
    }

    println!("=================================");
}

