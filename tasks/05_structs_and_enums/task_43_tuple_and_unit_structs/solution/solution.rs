struct Color(u8, u8, u8);
struct Point3D(i32, i32, i32);
struct SecurityKey;

fn main() {
    println!("=== GRAPHICS CANVAS & TOKEN SYSTEM ===");

    let sky_blue = Color(135, 206, 235);
    println!("Sky Blue Color:");
    println!("  Red:   {}", sky_blue.0);
    println!("  Green: {}", sky_blue.1);
    println!("  Blue:  {}", sky_blue.2);

    let cursor = Point3D(100, 250, -50);
    let Point3D(x, y, z) = cursor;
    println!("\n3D Cursor Position:");
    println!("  Coordinates: X={}, Y={}, Z={}", x, y, z);

    let _token = SecurityKey;
    println!("\nSecurity Token:");
    println!("  SecurityKey validated! (0-byte marker)");

    println!("======================================");
}

