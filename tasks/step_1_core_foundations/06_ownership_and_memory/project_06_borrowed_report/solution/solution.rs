enum StudyKind {
    Practice,
    Review,
}

struct Session {
    subject: String,
    minutes: u32,
    kind: StudyKind,
}

impl Session {
    fn new(subject: String, minutes: u32, kind: StudyKind) -> Self {
        Self { subject, minutes, kind }
    }

    fn description(&self) -> String {
        let kind_name = match &self.kind {
            StudyKind::Practice => "practice",
            StudyKind::Review => "review",
        };
        format!("{} - {} min ({})", self.subject, self.minutes, kind_name)
    }
}
fn total_minutes(sessions: &[Session]) -> u64 {
    let mut total = 0_u64;
    for session in sessions {
        total += session.minutes as u64;
    }
    total
}
fn longest_subject(sessions: &[Session]) -> Option<&str> {
    let mut longest: Option<&str> = None;
    let mut longest_count = 0;
    for session in sessions {
        let subject = session.subject.as_str();
        let count = subject.chars().count();
        if longest.is_none() || count > longest_count {
            longest = Some(subject);
            longest_count = count;
        }
    }
    longest
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
    let sessions = vec![
        Session::new(String::from("Rust"), 25, StudyKind::Practice),
        Session::new(String::from("Rust"), 30, StudyKind::Practice),
        Session::new(String::from("Review"), 20, StudyKind::Review),
    ];
    assert_eq!(total_minutes(&[]), 0);
    assert_eq!(remaining_minutes(total_minutes(&[]), daily_goal), 90);
    assert_eq!(longest_subject(&[]), None);
    assert_eq!(total_minutes(&sessions), 75);
    assert_eq!(longest_subject(&sessions), Some("Review"));
    let unicode = vec![
        Session::new(String::from("éé"), 1, StudyKind::Practice),
        Session::new(String::from("abc"), 1, StudyKind::Review),
        Session::new(String::from("xyz"), 1, StudyKind::Practice),
    ];
    assert_eq!(longest_subject(&unicode), Some("abc"));
    let blank = vec![Session::new(String::new(), 1, StudyKind::Review)];
    assert_eq!(longest_subject(&blank), Some(""));
    assert_eq!(total_minutes(&sessions[0..2]), 55);
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
    for session in &sessions {
        println!("{}. {}", number, session.description());
        number += 1;
    }
    println!("Total: {} min", total);
    println!("Progress: {}", progress_status(total, daily_goal));
    println!("Remaining: {} min", remaining_minutes(total, daily_goal));
    match longest_subject(&sessions) {
        Some(subject) => println!("Longest subject: {}", subject),
        None => println!("Longest subject: none"),
    }
    println!("Sessions still available: {}", sessions.len());
    println!("====================");
}
