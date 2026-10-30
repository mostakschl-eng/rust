#[derive(Debug)]
struct DeviceSpec {
    model: String,
    storage_gb: u32,
    ram_gb: u32,
    has_5g: bool,
}

fn main() {
    println!("=== PHONE HARDWARE CONFIGURATOR ===");

    let phone_base = DeviceSpec {
        model: String::from("Pixel Standard"),
        storage_gb: 128,
        ram_gb: 8,
        has_5g: true,
    };

    println!("Base Model: {} ({}GB Storage, {}GB RAM, 5G: {})", 
        phone_base.model, phone_base.storage_gb, phone_base.ram_gb, phone_base.has_5g);

    let phone_pro = DeviceSpec {
        model: String::from("Pixel Pro"),
        storage_gb: 256,
        ..phone_base
    };

    println!("\nPro Model (Upgraded Storage & Model via ..):");
    println!("Model:      {}", phone_pro.model);
    println!("Storage:    {} GB", phone_pro.storage_gb);
    println!("RAM:        {} GB", phone_pro.ram_gb);
    println!("5G Support: {}", phone_pro.has_5g);
    println!("\nFull Debug:\n{:?}", phone_pro);

    println!("===================================");
}
