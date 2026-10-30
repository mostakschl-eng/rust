fn main() {
    println!("=== MEMORY ARCHITECTURE INSPECTOR ===");

    // TODO 1: Declare stack variables: age (25), rating (4.9), is_pro (true)
    let age: i32 = 25;
    let rating: f64 = 4.9;
    let is_pro: bool = true;

    println!("[STACK MEMORY]");
    println!("Age:     {} years", age);
    println!("Rating:  {:.2} / 5.0", rating);
    println!("Pro Dev: {}", is_pro);

    // TODO 2: Declare mutable String bio and append " & System Architect"
    let mut bio = String::from("Rust Explorer");
    bio.push_str(" & System Architect");

    // TODO 3: Declare mutable Vec<i32> skill_levels and push 99
    let mut skill_levels = vec![85, 90, 95];
    skill_levels.push(99);

    println!("\n[HEAP MEMORY]");
    println!("Bio:     {}", bio);
    println!("Skills:  {:?} (Count: {})", skill_levels, skill_levels.len());

    println!("=====================================");
}

