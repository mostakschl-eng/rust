fn main() {
    println!("=== WEATHER STATION 5-DAY LOG ===");

    let weekdays: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let temperatures: [i32; 5] = [28, 31, 30, 27, 29];

    println!("{}: {}°C", weekdays[0], temperatures[0]);
    println!("{}: {}°C", weekdays[4], temperatures[4]);
    println!("Total days logged: {}", weekdays.len());

    let sensor_status = [1; 5];
    println!("Sensor Check: {:?}", sensor_status);

    println!("=================================");
}

