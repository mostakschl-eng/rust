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
fn total_minutes(sessions: &Vec<Session>) -> u64 {
    let mut total = 0_u64;
    for session in sessions {
        total += session.minutes as u64;
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
    let sessions = vec![
        Session::new(String::from("Rust"), 25, StudyKind::Practice),
        Session::new(String::from("Rust"), 30, StudyKind::Practice),
        Session::new(String::from("Review"), 20, StudyKind::Review),
    ];
    let empty: Vec<Session> = Vec::new();
    assert_eq!(total_minutes(&empty), 0);
    assert_eq!(remaining_minutes(total_minutes(&empty), daily_goal), 90);
    assert_eq!(total_minutes(&sessions), 75);
    assert_eq!(sessions[0].description(), "Rust - 25 min (practice)");
    assert_eq!(sessions[2].description(), "Review - 20 min (review)");
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
    println!("====================");
}
