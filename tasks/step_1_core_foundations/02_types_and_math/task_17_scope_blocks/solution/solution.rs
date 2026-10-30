// Official Solution for Task 17: Dungeon Treasure Room (Scope & Code Blocks)

fn main() {
    let gold = 100;

    println!("=== DUNGEON ADVENTURE ===");
    println!("Outside dungeon:   Gold = {}", gold);

    println!("[Entered Secret Vault]");
    {
        let dungeon_chest = 500;
        let gold = gold + dungeon_chest; // Shadows outer gold inside this room only
        println!("Inside secret room: Gold = {}", gold);
    }
    println!("[Exited Secret Vault]");

    println!("Back outside room: Gold = {}", gold);
    println!("=========================");
}

