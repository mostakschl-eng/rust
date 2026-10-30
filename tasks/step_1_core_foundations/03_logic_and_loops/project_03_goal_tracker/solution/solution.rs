const DAILY_GOAL: i32 = 90;

fn print_header() {
    println!("=== Study Tracker ===");
}

fn print_footer() {
    println!("Keep learning, one session at a time.");
}

// Codes: 0 below, 1 exactly at, 2 above the goal.
fn progress_level(total: i32, goal: i32) -> i32 {
    if total < goal {
        0
    } else if total == goal {
        1
    } else {
        2
    }
}

// shortcut: fixed small nonnegative inputs, add validation when accepting user data.
fn remaining_minutes(total: i32, goal: i32) -> i32 {
    if total < goal {
        goal - total
    } else {
        0
    }
}

fn main() {
    assert_eq!(progress_level(0, DAILY_GOAL), 0);
    assert_eq!(progress_level(89, DAILY_GOAL), 0);
    assert_eq!(progress_level(90, DAILY_GOAL), 1);
    assert_eq!(progress_level(91, DAILY_GOAL), 2);
    assert_eq!(remaining_minutes(0, DAILY_GOAL), 90);
    assert_eq!(remaining_minutes(75, DAILY_GOAL), 15);
    assert_eq!(remaining_minutes(90, DAILY_GOAL), 0);
    assert_eq!(remaining_minutes(91, DAILY_GOAL), 0);
    let sessions = 3;
    let minutes_each = 25;
    let mut total = 0;
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    println!("Sessions: {} x {} minutes", sessions, minutes_each);
    for session in 1..=sessions {
        total += minutes_each;
        println!("Session {}: {} minutes", session, minutes_each);
    }
    assert_eq!(total, 75);
    println!("Studied: {} minutes", total);
    println!("Remaining: {} minutes", remaining_minutes(total, DAILY_GOAL));
    let level = progress_level(total, DAILY_GOAL);
    if level == 0 {
        println!("Progress: below goal");
    } else if level == 1 {
        println!("Progress: goal reached");
    } else {
        println!("Progress: above goal");
    }
    print_footer();
}
