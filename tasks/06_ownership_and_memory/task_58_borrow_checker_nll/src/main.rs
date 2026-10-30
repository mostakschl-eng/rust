fn main() {
    println!("=== ALCHEMIST POTION SHELF ===");

    let mut shelf = vec!["Health Potion", "Mana Potion"];

    // TODO 1: Borrow shelf[0] into active_potion and print it
    let active_potion = &shelf[0];
    println!("Currently inspecting: {}", active_potion);
    // Under NLL, active_potion is now expired!

    println!("Adding new rare potion to shelf...");
    // TODO 2: Push "Elixir of Life" to shelf
    shelf.push("Elixir of Life");

    println!("Shelf Inventory: {:?}", shelf);

    println!("==============================");
}

