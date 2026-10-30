use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const DAILY_GOAL: u64 = 90;
const LOG_FILE: &str = "study_sessions.tsv";

#[derive(Debug, PartialEq)]
enum StudyKind { Practice, Review }

impl StudyKind {
    fn label(&self) -> &str {
        match self { Self::Practice => "practice", Self::Review => "review" }
    }
}

#[derive(Debug)]
struct Session {
    subject: String,
    minutes: u32,
    kind: StudyKind,
}

fn make_session(subject: &str, minutes: &str, kind: &str) -> Result<Session, String> {
    if subject.split_whitespace().next() != Some(subject) {
        return Err(String::from("Subject must be one nonempty word."));
    }
    let minutes = match minutes.parse::<u32>() {
        Ok(value) if value > 0 && value <= 1440 => value,
        _ => return Err(String::from("Minutes must be a whole number from 1 to 1440.")),
    };
    let kind = match kind {
        "practice" => StudyKind::Practice,
        "review" => StudyKind::Review,
        _ => return Err(String::from("Kind must be practice or review.")),
    };
    Ok(Session { subject: subject.to_string(), minutes, kind })
}

fn parse_record(line: &str) -> Result<Session, String> {
    let mut fields = line.split('\t');
    match (fields.next(), fields.next(), fields.next(), fields.next()) {
        (Some(subject), Some(minutes), Some(kind), None) => make_session(subject, minutes, kind),
        _ => Err(String::from("A saved record must have exactly three tab-separated fields.")),
    }
}

fn load_sessions(path: &str) -> io::Result<Vec<Session>> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    if !text.is_empty() && !text.ends_with('\n') {
        return Err(io::Error::new(io::ErrorKind::InvalidData,
            "The log has an unfinished final line. Inspect a copy before changing it."));
    }
    let mut sessions = Vec::new();
    for (index, line) in text.lines().enumerate() {
        match parse_record(line) {
            Ok(session) => sessions.push(session),
            Err(message) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid saved line {}: {}", index + 1, message))),
        }
    }
    Ok(sessions)
}

fn append_session(path: &str, session: &Session) -> io::Result<()> {
    // shortcut: one process owns this log; add file locking before sharing writers.
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let row = format!("{}\t{}\t{}\n", session.subject, session.minutes, session.kind.label());
    file.write_all(row.as_bytes())
}

fn goal_status(total: u64, goal: u64) -> &'static str {
    if total < goal { "below goal" } else if total == goal { "goal reached" } else { "above goal" }
}

fn longest_subject(sessions: &[Session]) -> Option<&str> {
    let mut longest: Option<&str> = None;
    for session in sessions {
        match longest {
            Some(current) if current.chars().count() >= session.subject.chars().count() => {},
            _ => longest = Some(session.subject.as_str()),
        }
    }
    longest
}

fn report_text(sessions: &[Session], goal: u64) -> String {
    let mut total = 0_u64;
    for session in sessions { total += u64::from(session.minutes); }
    let remaining = if total < goal { goal - total } else { 0 };
    format!("Sessions: {}\nTotal minutes: {}\nGoal status: {}\nRemaining minutes: {}\nLongest subject: {}",
        sessions.len(), total, goal_status(total, goal), remaining, longest_subject(sessions).unwrap_or("none"))
}

fn list_sessions(sessions: &[Session]) {
    if sessions.is_empty() { println!("No sessions yet."); }
    for (index, session) in sessions.iter().enumerate() {
        println!("{}. {}: {} minutes ({})", index + 1, session.subject, session.minutes, session.kind.label());
    }
}

fn print_help() {
    println!("Commands: add <subject> <minutes> <practice|review> | list | report | help | quit");
}

fn check_logic() {
    let valid = parse_record("Rust\t25\tpractice").expect("the fixed sample should parse");
    assert_eq!(valid.minutes, 25);
    assert_eq!(valid.kind, StudyKind::Practice);
    for invalid in ["Rust\t0\tpractice", "Rust\t1441\treview", "Rust\t-1\tpractice",
        "Rust\t4294967296\treview", "Rust\tabc\tpractice", "Rust\t25\tother",
        "\t25\tpractice", "Rust Notes\t25\tpractice", "Rust\t25\tpractice\textra"] {
        assert!(parse_record(invalid).is_err());
    }
    assert_eq!(goal_status(89, 90), "below goal");
    assert_eq!(goal_status(90, 90), "goal reached");
    assert_eq!(goal_status(91, 90), "above goal");
    assert_eq!(longest_subject(&[]), None);
    assert!(report_text(&[], 90).contains("Total minutes: 0"));
    let names = vec![make_session("কক", "1", "review").unwrap(),
        make_session("Rust", "1", "practice").unwrap()];
    assert_eq!(longest_subject(&names), Some("Rust"));
}

fn main() -> io::Result<()> {
    check_logic();
    let mut sessions = load_sessions(LOG_FILE)?;
    println!("=== STUDY TRACKER ===");
    println!("Learner: Rahim");
    println!("Daily goal: {} minutes", DAILY_GOAL);
    println!("Loaded sessions: {}", sessions.len());
    print_help();
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 { break; }
        let mut words = input.split_whitespace();
        match words.next() {
            None => continue,
            Some("add") => {
                let parsed = match (words.next(), words.next(), words.next(), words.next()) {
                    (Some(subject), Some(minutes), Some(kind), None) => make_session(subject, minutes, kind),
                    _ => Err(String::from("Use: add <subject> <minutes> <practice|review>")),
                };
                match parsed {
                    Ok(session) => {
                        append_session(LOG_FILE, &session)?;
                        println!("Saved: {} ({} minutes, {})", session.subject, session.minutes, session.kind.label());
                        sessions.push(session);
                    }
                    Err(message) => println!("Input error: {}", message),
                }
            }
            Some("list") if words.next().is_none() => list_sessions(&sessions),
            Some("report") if words.next().is_none() => println!("{}", report_text(&sessions, DAILY_GOAL)),
            Some("help") if words.next().is_none() => print_help(),
            Some("quit") if words.next().is_none() => break,
            _ => println!("Input error: unknown command or extra arguments. Type help."),
        }
    }
    println!("Goodbye.");
    Ok(())
}
