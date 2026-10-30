// TODO 1: Define Color tuple struct with (u8, u8, u8)
struct Color(u8, u8, u8);

// TODO 2: Define Point3D tuple struct with (i32, i32, i32)
struct Point3D(i32, i32, i32);

// TODO 3: Define SecurityKey unit-like struct
struct SecurityKey;

fn main() {
    println!("=== GRAPHICS CANVAS & TOKEN SYSTEM ===");

    // TODO 4: Instantiate sky_blue and print channels via .0, .1, .2
    let sky_blue = Color(135, 206, 235);
    println!("Sky Blue Color:");
    println!("  Red:   {}", sky_blue.0);
    println!("  Green: {}", sky_blue.1);
    println!("  Blue:  {}", sky_blue.2);

    // TODO 5: Instantiate cursor and destructure Point3D(x, y, z)
    let cursor = Point3D(100, 250, -50);
    let Point3D(x, y, z) = cursor;
    println!("\n3D Cursor Position:");
    println!("  Coordinates: X={}, Y={}, Z={}", x, y, z);

    // TODO 6: Instantiate SecurityKey and print validation
    let _token = SecurityKey;
    println!("\nSecurity Token:");
    println!("  SecurityKey validated! (0-byte marker)");

    println!("======================================");
}

