fn main() {
    println!("=== SYSTEM CONFIGURATION LOADER ===");

    // TODO 1: custom_theme unwrap_or "Cyberpunk Dark"
    let custom_theme: Option<&str> = None;
    let active_theme = custom_theme.unwrap_or("Cyberpunk Dark");
    println!("Active Theme:      {} (Defaulted safely)", active_theme);

    // TODO 2: cached_port unwrap_or_else || 8080
    let cached_port: Option<u16> = None;
    let service_port = cached_port.unwrap_or_else(|| 8080);
    println!("Service Port:      {} (Computed lazily)", service_port);

    // TODO 3: root_cert expect
    let root_cert: Option<&str> = Some("CERT_SHA256_ROOT_OK");
    let cert = root_cert.expect("CRITICAL: Root certificate missing from bundle!");
    println!("Security Cert:     {} (Verified via expect)", cert);

    println!("===================================");
}

