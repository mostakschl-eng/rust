// TODO 1: Implement copy_primitive(val: i32)
fn copy_primitive(val: i32) {
    println!("Inside copy_primitive: {}", val);
}

// TODO 2: Implement consume_string(text: String)
fn consume_string(text: String) {
    println!("Inside consume_string: {} (Taking full ownership)", text);
}

fn main() {
    println!("=== MOVE VS COPY DISPATCHER ===");

    println!("[1. Testing Primitive Copy]");
    let original_num: i32 = 500;
    copy_primitive(original_num);
    println!("Still alive in main:   {} (Copy trait preserved it!)", original_num);

    println!("\n[2. Testing Heap Move]");
    let parcel = String::from("Rocket Fuel");
    consume_string(parcel);
    // parcel is moved! We do not access it anymore.
    println!("Parcel ownership successfully moved and cleaned up.");

    println!("===============================");
}

