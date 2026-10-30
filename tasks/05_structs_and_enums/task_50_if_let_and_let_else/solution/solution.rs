fn verify_clearance(badge_id: Option<u32>) {
    let Some(id) = badge_id else {
        println!("⛔ Access Denied: No badge detected. Incident logged.");
        return;
    };

    println!("🔑 Access Approved for Officer #{}!", id);
}

fn main() {
    println!("=== CONCISE MATCHING TERMINAL ===");

    let prize: Option<&str> = Some("Diamond Trophy 💎");

    if let Some(trophy) = prize {
        println!("🎉 Winner Claimed: {}", trophy);
    } else {
        println!("No prize awarded.");
    }

    println!("\n=== SECURITY CLEARANCE CHECKPOINT ===");

    println!("Attempt 1 (Valid Badge):");
    verify_clearance(Some(9042));

    println!("\nAttempt 2 (Missing Badge):");
    verify_clearance(None);

    println!("=====================================");
}

