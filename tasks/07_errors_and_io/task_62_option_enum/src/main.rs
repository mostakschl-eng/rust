// TODO 1: Implement find_player_score(player_id: u32) -> Option<u32>
fn find_player_score(player_id: u32) -> Option<u32> {
    match player_id {
        101 => Some(950),
        102 => Some(820),
        _ => None,
    }
}

fn main() {
    println!("=== GAME SCORE LOOKUP SERVICE ===");

    println!("Querying Player #101:");
    let res101 = find_player_score(101);
    println!("Found record? {}", res101.is_some());
    match res101 {
        Some(score) => println!("Player 101 Score: {} pts", score),
        None => println!("Player 101: No record found."),
    }

    println!("\nQuerying Player #999:");
    let res999 = find_player_score(999);
    match res999 {
        Some(score) => println!("Player 999 Score: {} pts", score),
        None => println!("Player 999: No record found."),
    }

    println!("=================================");
}

