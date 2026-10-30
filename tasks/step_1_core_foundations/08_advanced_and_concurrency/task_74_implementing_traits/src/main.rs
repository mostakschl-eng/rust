use std::f64::consts::PI;

pub trait Area {
    fn area(&self) -> f64;
}

#[allow(dead_code)]
struct Circle {
    radius: f64,
}

#[allow(dead_code)]
struct Rectangle {
    width: f64,
    height: f64,
}

// TODO 1: Implement Area for Circle (Area = PI * radius * radius)
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// TODO 2: Implement Area for Rectangle (Area = width * height)
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// TODO 3: Implement compare_shapes with a `where` clause
fn compare_shapes<T, U>(shape_a: &T, shape_b: &U)
where
    T: Area,
    U: Area,
{
    let area_a = shape_a.area();
    let area_b = shape_b.area();

    println!("Shape A (Circle):    {:.2} sq units", area_a);
    println!("Shape B (Rectangle): {:.2} sq units", area_b);

    if area_a > area_b {
        println!("Result: Shape A is larger! 📐");
    } else if area_b > area_a {
        println!("Result: Shape B is larger! 📐");
    } else {
        println!("Result: Both shapes have equal area! 📐");
    }
}

fn main() {
    println!("=== GEOMETRIC SHAPE COMPARATOR ===");

    // TODO 4: Instantiate Circle (radius 5.0) and Rectangle (width 10.0, height 8.0)
    let my_circle = Circle { radius: 5.0 };
    let my_rect = Rectangle {
        width: 10.0,
        height: 8.0,
    };

    // TODO 5: Call compare_shapes(&my_circle, &my_rect)
    compare_shapes(&my_circle, &my_rect);

    println!("==================================");
}

