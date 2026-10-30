use std::collections::HashMap;

fn main() {
    println!("=== FRESH MARKET PRICE CATALOG ===");

    let mut prices = HashMap::new();

    prices.insert("Apple", 2.50);
    prices.insert("Banana", 1.20);
    prices.insert("Mango", 3.75);

    println!("Initial catalog items: {}", prices.len());

    println!("Has 'Banana'? {}", prices.contains_key("Banana"));

    match prices.get("Mango") {
        Some(price) => println!("Lookup [Mango]: Price of Mango: ${:.2}", price),
        None => println!("Sorry, Mango is not in stock!"),
    }

    match prices.get("Dragonfruit") {
        Some(price) => println!("Lookup [Dragonfruit]: Price of Dragonfruit: ${:.2}", price),
        None => println!("Lookup [Dragonfruit]: Sorry, Dragonfruit is not in stock!"),
    }

    println!("Adding Orange via entry API...");
    prices.entry("Orange").or_insert(1.90);

    println!("Updated catalog count: {}", prices.len());

    println!("==================================");
}

