// Checkpoint 6: report through borrowed views; keep the records owned by main.
fn main() {
    let learner = "Rahim";
    let daily_goal = 90_u64;
    println!("Study Tracker for {} | Goal: {} min", learner, daily_goal);

    // TODO 1: Rebuild checkpoint 5's Session, StudyKind, constructor, and description.
    // TODO 2: Change total_minutes to accept &[Session], then restore the report.
    // TODO 3: Add longest_subject(&[Session]) -> Option<&str> without cloning.
    // TODO 4: Compare subject.chars().count(); keep the first equal-length subject.
    // TODO 5: Return None for no sessions, Some("") for a present empty subject.
    // TODO 6: Match the result, then print sessions.len() to show ownership remains.
    // TODO 7: Add silent assertions for empty data, Unicode, ties, a sub-slice, and goals.
}
