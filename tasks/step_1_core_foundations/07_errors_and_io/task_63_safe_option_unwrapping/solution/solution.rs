fn main() {
    println!("=== SYSTEM CONFIGURATION LOADER ===");

    let custom_theme: Option<&str> = None;
    let active_theme = custom_theme.unwrap_or("Cyberpunk Dark");
    println!("Active Theme:      {} (Defaulted safely)", active_theme);

    let cached_port: Option<u16> = None;
    let service_port = cached_port.unwrap_or_else(|| 8080);
    println!("Service Port:      {} (Computed lazily)", service_port);

    let root_cert: Option<&str> = Some("CERT_SHA256_ROOT_OK");
    let cert = root_cert.expect("CRITICAL: Root certificate missing from bundle!");
    println!("Security Cert:     {} (Verified via expect)", cert);

    println!("===================================");
}

