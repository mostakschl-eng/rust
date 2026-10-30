fn main() {
    println!("=== AIR CARGO SHIPMENT LABEL ===");

    let parcel = (10482, "Tokyo", 4.75, false);

    println!("Tracking ID:  #{}", parcel.0);
    println!("Destination:  {}", parcel.1);
    println!("Weight:       {} kg", parcel.2);
    println!("Delivered?    {}", parcel.3);
    println!("Raw Parcel:   {:?}", parcel);

    println!("================================");
}

