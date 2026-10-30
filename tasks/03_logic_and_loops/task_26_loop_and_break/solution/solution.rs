// Official Solution for Task 26: EV Battery Charger Station (`loop` and `break`)

fn main() {
    println!("EV Station: Plugged in. Fast charging started...");
    let mut battery_level = 70;

    loop {
        battery_level += 10;
        println!("Charging: {}%...", battery_level);

        if battery_level >= 100 {
            println!("Battery 100% Full! Unplugging cable. ⚡");
            break;
        }
    }

    println!("EV Station: Ready to drive!");
}

