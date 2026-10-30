mod security {
    // Private helper:
    fn hash_token(input: &str) -> String {
        format!("HASH_{}", input)
    }

    // Public function:
    pub fn verify_credentials(user: &str, token: &str) -> bool {
        let hashed = hash_token(token);
        user == "Admin" && hashed == "HASH_SECRET_99"
    }
}

mod telemetry {
    pub struct Sensor {
        pub id: u32,
        pub label: String,
        calibration_offset: f64, // Private field!
    }

    impl Sensor {
        pub fn new(id: u32, label: &str) -> Self {
            Self {
                id,
                label: label.to_string(),
                calibration_offset: 1.32,
            }
        }

        pub fn measure(&self) -> f64 {
            100.0 + self.calibration_offset
        }
    }
}

// TODO: Bring items into scope with use
use security::verify_credentials;
use telemetry::Sensor;

fn main() {
    println!("=== FORTRESS MODULAR SYSTEM ===");

    println!("[1. Testing Security Module]");
    let is_auth = verify_credentials("Admin", "SECRET_99");
    println!("Credentials verified: {}", is_auth);

    println!("\n[2. Testing Telemetry Module]");
    let sensor = Sensor::new(401, "Pressure Gauge");
    println!("Sensor #{} ({}): Reading = {:.2} kPa", sensor.id, sensor.label, sensor.measure());
    println!("Modular boundaries enforced cleanly! 🛡️");

    println!("===============================");
}

