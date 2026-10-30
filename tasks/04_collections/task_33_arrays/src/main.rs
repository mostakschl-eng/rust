fn main() {
    println!("=== WEATHER STATION 5-DAY LOG ===");

    // TODO 1: Declare weekdays array of 5 days
    let weekdays: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // TODO 2: Declare temperatures array of 5 numbers: [28, 31, 30, 27, 29]
    let temperatures: [i32; 5] = [28, 31, 30, 27, 29];

    // TODO 3: Print Monday and Friday temperatures
    println!("{}: {}°C", weekdays[0], temperatures[0]);
    println!("{}: {}°C", weekdays[4], temperatures[4]);

    // TODO 4: Print total logged days using .len()
    println!("Total days logged: {}", weekdays.len());

    // TODO 5: Create sensor_status array filled with 5 ones using repetition syntax [1; 5]
    let sensor_status = [1; 5];
    println!("Sensor Check: {:?}", sensor_status);

    println!("=================================");
}

