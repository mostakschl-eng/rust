fn main() {
    println!("=== SACRED OWNERSHIP DEMO ===");

    println!("Entering temporary room...");
    // TODO 1: Create inner scope with temporary_pass
    {
        let temporary_pass = String::from("VIP-ACCESS-PASS-2024");
        println!("Inside inner scope: {}", temporary_pass);
    }
    println!("Left temporary room! (VIP pass automatically dropped by Rust)");

    println!("\nOwnership Transfer Relay:");
    // TODO 2: Create original_owner and transfer to new_owner
    let original_owner = String::from("Championship Cup 🏆");
    let new_owner = original_owner;

    println!("Trophy successfully transferred to new owner: {}", new_owner);

    println!("=============================");
}

