const DAILY_GOAL: i32 = 90;

fn print_header() {
    println!("=== Study Tracker ===");
}

fn print_footer() {
    println!("Keep learning, one session at a time.");
}

fn main() {
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    print_footer();
}
