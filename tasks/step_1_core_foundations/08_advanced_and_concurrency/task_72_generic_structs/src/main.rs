#[derive(Debug)]
struct Coordinate<T, U> {
    x: T,
    y: U,
}

// TODO 1: Implement new, x, and y in impl<T, U> Coordinate<T, U>
impl<T, U> Coordinate<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    println!("=== GENERIC COORDINATE MATRIX ===");

    // TODO 2: Create integer, GPS, and mixed coordinates
    let int_coord = Coordinate::new(10, 20);
    let gps_coord = Coordinate::new(23.75, 90.38);
    let mixed_coord = Coordinate::new("Section-Alpha", 42);

    println!("Integer Grid:  X={}, Y={}", int_coord.x(), int_coord.y());
    println!("GPS Telemetry: X={:.2}, Y={:.2}", gps_coord.x(), gps_coord.y());
    println!("Mixed Beacon:  X={:?}, Y={}", mixed_coord.x(), mixed_coord.y());

    println!("=================================");
}

