const DAILY_GOAL: i32 = 90;

fn print_header() {
    println!("=== Study Tracker ===");
}

fn print_footer() {
    println!("Keep learning, one session at a time.");
}

// shortcut: fixed small nonnegative inputs, add validation when accepting user data.
fn study_minutes(sessions: i32, minutes_each: i32) -> i32 {
    sessions * minutes_each
}

fn main() {
    let sessions = 3;
    let minutes_each = 25;
    let total = study_minutes(sessions, minutes_each);
    let remaining = DAILY_GOAL - total;
    assert_eq!(total, 75);
    assert_eq!(remaining, 15);
    assert_eq!(study_minutes(0, 25), 0);
    assert_eq!(study_minutes(3, 0), 0);
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    println!("Sessions: {} x {} minutes", sessions, minutes_each);
    println!("Studied: {} minutes", total);
    println!("Remaining: {} minutes", remaining);
    print_footer();
}
