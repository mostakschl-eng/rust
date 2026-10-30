fn main() {
    println!("=== AIR CARGO SHIPMENT LABEL ===");

    // TODO 1: Create a tuple named `parcel` with:
    //         - ID: 10482
    //         - Destination: "Tokyo"
    //         - Weight: 4.75
    //         - Delivered: false
    // let parcel = (...);

    // TODO 2: Print each field using parcel.0, parcel.1, etc.
    // TODO 3: Print the full raw tuple using {:?}

    // Starter placeholder:
    let sample = (1, "Sample", 0.0, true);
    println!("Tracking ID:  #{}", sample.0);
    println!("Raw Parcel:   {:?}", sample);

    println!("================================");
}

