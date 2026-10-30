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

    println!("Base Model:\n{:?}", phone_base);

    let phone_pro = DeviceSpec {
        model: String::from("Pixel Pro"),
        storage_gb: 256,
        ..phone_base
    };

    println!("\nPro Model (Upgraded Storage & Model via ..):");
    println!("{:?}", phone_pro);

    println!("===================================");
}
