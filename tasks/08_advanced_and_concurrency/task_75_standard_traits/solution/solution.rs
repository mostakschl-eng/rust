use std::fmt;

#[allow(dead_code)]
struct Money {
    cents: u64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = self.cents % 100;
        write!(f, "${}.{:02}", dollars, cents)
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
struct AudioSettings {
    volume: u8,
    enable_surround: bool,
    sample_rate: u32,
}

fn main() {
    println!("=== SYSTEM CONFIG & PRICING ===");

    let item_price = Money { cents: 2450 };
    println!("Item Price:     {}", item_price);

    let default_audio = AudioSettings::default();
    println!("Default Audio:  {:?}", default_audio);

    let custom_audio = AudioSettings {
        volume: 85,
        ..Default::default()
    };
    println!("Custom Audio:   {:?}", custom_audio);

    println!("===============================");
}

