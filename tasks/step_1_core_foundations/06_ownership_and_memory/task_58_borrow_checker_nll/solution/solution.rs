fn main() {
    println!("=== ALCHEMIST POTION SHELF ===");

    let mut shelf = vec!["Health Potion", "Mana Potion"];

    let active_potion = &shelf[0];
    println!("Currently inspecting: {}", active_potion);

    println!("Adding new rare potion to shelf...");
    shelf.push("Elixir of Life");

    println!("Shelf Inventory: {:?}", shelf);

    println!("==============================");
}

