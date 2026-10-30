struct UserProfile {
    username: String,
    email: String,
    middle_name: Option<String>,
    phone_number: Option<String>,
}

fn main() {
    println!("=== USER REGISTRATION PROFILE ===");

    let user = UserProfile {
        username: String::from("Farhan"),
        email: String::from("farhan@tech.com"),
        middle_name: None,
        phone_number: Some(String::from("+880-1711-000000")),
    };

    println!("Username:    {}", user.username);
    println!("Email:       {}", user.email);

    match &user.middle_name {
        Some(m) => println!("Middle Name: {}", m),
        None => println!("Middle Name: [Not Provided]"),
    }

    match &user.phone_number {
        Some(p) => println!("Phone:       {}", p),
        None => println!("Phone:       [Not Provided]"),
    }

    println!("=================================");
}

