fn inspect_cargo<T: std::fmt::Debug>(category: &str, item: T) {
    println!("[CARGO SCANNER: {}] Item Details: {:?}", category, item);
}

fn main() {
    println!("=== UNIVERSAL CARGO SCANNER ===");

    inspect_cargo("Hardware ID", 1048);
    inspect_cargo("Core Temperature", 98.6);
    inspect_cargo("Mission Codename", "APOLLO-11");
    inspect_cargo("Safety Clearance", true);

    println!("===============================");
}

