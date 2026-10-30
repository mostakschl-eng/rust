fn main() {
    println!("=== MEMORY ARCHITECTURE INSPECTOR ===");

    let age: i32 = 25;
    let rating: f64 = 4.9;
    let is_pro: bool = true;

    println!("[STACK MEMORY]");
    println!("Age:     {} years", age);
    println!("Rating:  {:.2} / 5.0", rating);
    println!("Pro Dev: {}", is_pro);

    let mut bio = String::from("Rust Explorer");
    bio.push_str(" & System Architect");

    let mut skill_levels = vec![85, 90, 95];
    skill_levels.push(99);

    println!("\n[HEAP MEMORY]");
    println!("Bio:     {}", bio);
    println!("Skills:  {:?} (Count: {})", skill_levels, skill_levels.len());

    println!("=====================================");
}

