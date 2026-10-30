// Official Solution for Task 08: RPG Game Character Profile (Variables)

fn main() {
    let hero_name = "Warrior Rahim";
    let hero_level = 5;
    let hero_health = 100;
    let hero_gold = 250;

    let potion_cost = 50;
    let remaining_gold = hero_gold - potion_cost;

    println!("=== CHARACTER PROFILE ===");
    println!("Hero Name:         {}", hero_name);
    println!("Current Level:     {}", hero_level);
    println!("Health Points:     {} HP", hero_health);
    println!("Starting Gold:     {} coins", hero_gold);
    println!("Bought healing potion for {} coins!", potion_cost);
    println!("Remaining Gold:    {} coins", remaining_gold);
    println!("=========================");
}
