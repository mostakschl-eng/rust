fn main() {
    println!("=== GROCERY CART MANAGER ===");

    let mut cart = Vec::new();

    // TODO 1: Push "Apples", "Bread", "Milk"
    cart.push("Apples");
    cart.push("Bread");
    cart.push("Milk");

    // TODO 2: Print cart count with .len()
    println!("Added items! Cart count: {}", cart.len());

    // TODO 3: Pop the last item and print it
    match cart.pop() {
        Some(item) => println!("Popped item from cart: {}", item),
        None => println!("Cart is empty!"),
    }

    println!("Remaining items: {:?}", cart);

    // TODO 4: Clear the cart with .clear()
    println!("Clearing all items...");
    cart.clear();

    // TODO 5: Check .is_empty()
    println!("Cart is empty? {}", cart.is_empty());
    if cart.is_empty() {
        println!("Cart successfully cleared! 🛒");
    }

    println!("============================");
}

