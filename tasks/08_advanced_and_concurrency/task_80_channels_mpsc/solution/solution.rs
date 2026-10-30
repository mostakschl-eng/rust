use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== SPACE STATION TELEMETRY HUB (MPSC) ===");
    println!("[HUB] Initializing sensor receivers...");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        tx1.send(String::from("Sensor Alpha: Temp 24.5 C")).unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(40));
        tx2.send(String::from("Sensor Beta:  Pressure 1013 hPa")).unwrap();
    });

    let tx3 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(60));
        tx3.send(String::from("Sensor Gamma: Humidity 55%")).unwrap();
    });

    drop(tx);

    for msg in rx {
        println!("   [HUB RECEIVE] {}", msg);
    }

    println!("[HUB] All telemetry data collected successfully! 🛰️");
    println!("==========================================");
}

