use std::fmt;

#[allow(dead_code)]
struct Money {
    cents: u64,
}

// TODO 1: Implement Display manually for Money
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = self.cents % 100;
        write!(f, "${}.{:02}", dollars, cents)
    }
}

// TODO 2: Derive Debug and Default on AudioSettings
#[derive(Debug, Default)]
#[allow(dead_code)]
struct AudioSettings {
    volume: u8,
    enable_surround: bool,
    sample_rate: u32,
}

fn main() {
    println!("=== SYSTEM CONFIG & PRICING ===");

    // TODO 3: Create Money with 2450 cents and print with {}
    let item_price = Money { cents: 2450 };
    println!("Item Price:     {}", item_price);

    // TODO 4: Create AudioSettings::default() and print with {:?}
    let default_audio = AudioSettings::default();
    println!("Default Audio:  {:?}", default_audio);

    // TODO 5: Create custom audio with volume 85 and ..Default::default(), print with {:?}
    let custom_audio = AudioSettings {
        volume: 85,
        ..Default::default()
    };
    println!("Custom Audio:   {:?}", custom_audio);

    println!("===============================");
}

