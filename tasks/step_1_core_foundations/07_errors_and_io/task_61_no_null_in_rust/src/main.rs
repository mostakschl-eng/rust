struct UserProfile {
    username: String,
    email: String,
    middle_name: Option<String>,
    phone_number: Option<String>,
}

fn main() {
    println!("=== USER REGISTRATION PROFILE ===");

    // TODO 1: Create user with None middle_name and Some phone_number
    let user = UserProfile {
        username: String::from("Farhan"),
        email: String::from("farhan@tech.com"),
        middle_name: None,
        phone_number: Some(String::from("+880-1711-000000")),
    };

    println!("Username:    {}", user.username);
    println!("Email:       {}", user.email);

    // TODO 2: Match middle_name
    match &user.middle_name {
        Some(m) => println!("Middle Name: {}", m),
        None => println!("Middle Name: [Not Provided]"),
    }

    // TODO 3: Match phone_number
    match &user.phone_number {
        Some(p) => println!("Phone:       {}", p),
        None => println!("Phone:       [Not Provided]"),
    }

    println!("=================================");
}

