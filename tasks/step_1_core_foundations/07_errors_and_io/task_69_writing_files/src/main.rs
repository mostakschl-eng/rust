use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("=== TELEMETRY FILE WRITER & LOGGER ===");

    let log_file = "server_log.txt";

    println!("Writing initial base configuration...");
    // TODO 1: Write initial line to log_file
    fs::write(log_file, "[CONFIG] Port=8080 | Mode=Production")?;

    println!("Appending runtime security event...");
    // TODO 2: Append event with OpenOptions
    let mut file = OpenOptions::new()
        .append(true)
        .open(log_file)?;

    writeln!(file, "\n[EVENT] Security Handshake Established")?;

    println!("\nReading final saved log from disk:");
    // TODO 3: Read back and print
    let saved_data = fs::read_to_string(log_file)?;
    println!("{}", saved_data.trim());

    println!("File operations complete! 💾");
    println!("======================================");

    Ok(())
}

