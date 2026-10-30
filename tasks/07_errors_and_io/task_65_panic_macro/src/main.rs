// TODO 1: Implement launch_sequence(fuel_level: u8, countdown: i32)
fn launch_sequence(fuel_level: u8, countdown: i32) {
    if fuel_level < 20 {
        panic!("EMERGENCY ABORT: Fuel level critical ({}%)!", fuel_level);
    }

    println!("Countdown reached T-minus {} seconds. Fuel: {}%.", countdown, fuel_level);
    println!("Engines ignite! Liftoff confirmed! 🚀");
}

fn main() {
    println!("=== ROCKET LAUNCH CONTROL CENTER ===");
    println!("Performing pre-flight safety check...");

    // Successful launch:
    launch_sequence(95, 0);

    // Optional: Uncomment below line to test panic emergency abort:
    // launch_sequence(10, 5);

    println!("====================================");
}

