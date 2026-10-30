fn main() {
    println!("=== GROCERY CART MANAGER ===");

    let mut cart = Vec::new();

    cart.push("Apples");
    cart.push("Bread");
    cart.push("Milk");

    println!("Added items! Cart count: {}", cart.len());

    match cart.pop() {
        Some(item) => println!("Popped item from cart: {}", item),
        None => println!("Cart is empty!"),
    }

    println!("Remaining items: {:?}", cart);

    println!("Clearing all items...");
    cart.clear();

    println!("Cart is empty? {}", cart.is_empty());
    if cart.is_empty() {
        println!("Cart successfully cleared! 🛒");
    }

    println!("============================");
}

