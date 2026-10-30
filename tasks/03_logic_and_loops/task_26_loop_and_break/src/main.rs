// Task 26: EV Battery Charger Station (`loop` and `break`)
//
// INSTRUCTIONS:
// 1. Declare `let mut battery_level = 70;`
// 2. Loop and increment by 10 until reaching 100%.
// 3. Break out of the loop and print departure message.
// 4. Test with `cargo run`.

fn main() {
    println!("EV Station: Plugged in. Fast charging started...");
    let mut battery_level = 70;
    println!("Initial battery: {}%", battery_level);

    // TODO: Write your loop with break statement here:
    loop {
        battery_level += 10;
        println!("Battery level: {}", battery_level);
        break;
    }

    println!("EV Station: Ready to drive!");
}
