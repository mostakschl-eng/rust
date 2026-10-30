// Official Solution for Task 19: Flight Telemetry Debugger (Debug Printing {:?} & {:#?})

fn main() {
    let flight_location = ("Flight 707", 32.7767, -96.7970, 35000);

    println!("=== FLIGHT TELEMETRY DEBUGGER ===");
    println!("Standard Debug: {:?}", flight_location);
    println!("Pretty Debug:\n{:#?}", flight_location);

    let airspeed_knots = 480 + 25;
    dbg!(airspeed_knots);

    println!("=================================");
}

