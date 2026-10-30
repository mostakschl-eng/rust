fn total_minutes(sessions: &Vec<(&str, u32)>) -> u64 {
    let mut total = 0_u64;
    for &(_subject, minutes) in sessions {
        total += minutes as u64;
    }
    total
}

fn remaining_minutes(total: u64, goal: u64) -> u64 {
    if total < goal { goal - total } else { 0 }
}

fn progress_status(total: u64, goal: u64) -> String {
    if total < goal {
        String::from("Below goal")
    } else if total == goal {
        String::from("Goal reached")
    } else {
        String::from("Above goal")
    }
}

fn main() {
    let learner = "Rahim";
    let daily_goal = 90_u64;
    let sessions: Vec<(&str, u32)> = vec![("Rust", 25), ("Rust", 30), ("Review", 20)];
    let empty: Vec<(&str, u32)> = Vec::new();
    assert_eq!(total_minutes(&empty), 0);
    assert_eq!(remaining_minutes(total_minutes(&empty), daily_goal), 90);
    assert_eq!(total_minutes(&sessions), 75);
    assert_eq!(remaining_minutes(75, 90), 15);
    assert_eq!(remaining_minutes(90, 90), 0);
    assert_eq!(remaining_minutes(100, 90), 0);
    assert_eq!(progress_status(75, 90), "Below goal");
    assert_eq!(progress_status(90, 90), "Goal reached");
    assert_eq!(progress_status(100, 90), "Above goal");

    let total = total_minutes(&sessions);
    println!("=== STUDY TRACKER ===");
    println!("Learner: {}", learner);
    println!("Daily goal: {} min", daily_goal);
    println!("Sessions:");
    let mut number = 1;
    for &(subject, minutes) in &sessions {
        println!("{}. {} - {} min", number, subject, minutes);
        number += 1;
    }
    println!("Total: {} min", total);
    println!("Progress: {}", progress_status(total, daily_goal));
    println!("Remaining: {} min", remaining_minutes(total, daily_goal));
    println!("====================");
}
