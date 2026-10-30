fn main() {
    println!("=== SERVER STATUS CODE LOOKUP ===");

    let codes = [101, 204, 301, 404, 500];

    // TODO 1: Print direct check for index 1 and index 3
    println!("Direct check [Index 1]: {}", codes[1]);
    println!("Direct check [Index 3]: {}", codes[3]);

    // TODO 2: Perform safe lookup at index 2 using .get(2)
    match codes.get(2) {
        Some(code) => println!("Lookup [Index 2]: Found status code: {}", code),
        None => println!("Lookup [Index 2]: Not found!"),
    }

    // TODO 3: Perform safe lookup at index 9 using .get(9)
    match codes.get(9) {
        Some(code) => println!("Lookup [Index 9]: Found status code: {}", code),
        None => println!("Lookup [Index 9]: ⚠️ Status code at index 9 does not exist! (Safely handled, zero crashes)"),
    }

    println!("=================================");
}

