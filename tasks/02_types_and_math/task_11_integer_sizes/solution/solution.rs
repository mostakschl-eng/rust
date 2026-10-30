// Official Solution for Task 11: System Specs & Data Sizes (Integer Types)

fn main() {
    let freezer_temp: i8 = -18;
    let battery_percentage: u8 = 95;
    let active_players: u32 = 1_250_000;
    let world_population: i64 = 8_000_000_000;

    println!("=== SYSTEM & WORLD STATS ===");
    println!("Freezer Temperature: {} C", freezer_temp);
    println!("Battery Level:       {}%", battery_percentage);
    println!("Online Players:      {}", active_players);
    println!("World Population:    {}", world_population);
    println!("============================");
}
