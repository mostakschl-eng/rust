fn main() {
    println!("=== MINEFIELD RADAR SCANNER ===");

    'radar_grid: for row in 1..=4 {
        for col in 1..=4 {
            println!("Radar scanning [Row {}, Col {}]...", row, col);

            if row == 3 && col == 2 {
                println!("⚠️ ALERT: Secret Treasure Detected at [3, 2]!");
                println!("Initiating emergency radar shutdown!");
                break 'radar_grid;
            }
        }
    }

    println!("Radar deactivated. Mission successful! 🏁");
    println!("===============================");
}

