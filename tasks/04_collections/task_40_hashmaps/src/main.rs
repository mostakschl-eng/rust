use std::collections::HashMap;

fn main() {
    println!("=== FRESH MARKET PRICE CATALOG ===");

    // TODO 1: Create a mutable HashMap
    let mut prices = HashMap::new();

    // TODO 2: Insert Apple (2.50), Banana (1.20), Mango (3.75)
    prices.insert("Apple", 2.50);
    prices.insert("Banana", 1.20);
    prices.insert("Mango", 3.75);

    println!("Initial catalog items: {}", prices.len());

    // TODO 3: Check contains_key for "Banana"
    println!("Has 'Banana'? {}", prices.contains_key("Banana"));

    // TODO 4: Query "Mango" safely with .get()
    match prices.get("Mango") {
        Some(price) => println!("Lookup [Mango]: Price of Mango: ${:.2}", price),
        None => println!("Sorry, Mango is not in stock!"),
    }

    // TODO 5: Query "Dragonfruit" safely with .get()
    match prices.get("Dragonfruit") {
        Some(price) => println!("Lookup [Dragonfruit]: Price of Dragonfruit: ${:.2}", price),
        None => println!("Lookup [Dragonfruit]: Sorry, Dragonfruit is not in stock!"),
    }

    // TODO 6: Insert Orange at 1.90 using entry().or_insert()
    println!("Adding Orange via entry API...");
    prices.entry("Orange").or_insert(1.90);

    println!("Updated catalog count: {}", prices.len());

    println!("==================================");
}

