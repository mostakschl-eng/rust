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

    launch_sequence(95, 0);
    // To observe the panic path instead, try launch_sequence(10, 5).

    println!("====================================");
}

