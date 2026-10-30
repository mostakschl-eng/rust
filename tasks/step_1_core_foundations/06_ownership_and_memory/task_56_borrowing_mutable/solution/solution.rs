fn add_alert_badge(msg: &mut String) {
    msg.push_str(" [CRITICAL LEVEL 5 ALERT 🚨]");
}

fn boost_voltage(voltage: &mut f64, multiplier: f64) {
    *voltage *= multiplier;
}

fn main() {
    println!("=== POWER GRID CONTROL CENTER ===");

    let mut system_status = String::from("Cooling Pump Fault");
    let mut grid_voltage = 220.0;

    println!("Original Status:  {}", system_status);
    println!("Original Voltage: {:.2} V", grid_voltage);

    println!("\nApplying emergency protocols...");
    add_alert_badge(&mut system_status);
    boost_voltage(&mut grid_voltage, 1.25);

    println!("Updated Status:   {}", system_status);
    println!("Updated Voltage:  {:.2} V", grid_voltage);

    println!("=================================");
}

