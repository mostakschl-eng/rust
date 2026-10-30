// Official Solution for Task 22: Cinema Ticket Booth (The `else if` Ladder)

fn main() {
    let age = 68;

    let category: &str;
    let price: i32;

    if age < 5 {
        category = "Toddler";
        price = 0;
    } else if age <= 12 {
        category = "Child";
        price = 8;
    } else if age <= 64 {
        category = "Adult Standard";
        price = 15;
    } else {
        category = "Senior Citizen";
        price = 10;
    }

    println!("=== CINEMA TICKET KIOSK ===");
    println!("Customer Age:       {} years", age);
    println!("Ticket Category:    {}", category);
    println!("Ticket Price:       ${}", price);
    println!("Enjoy the movie! 🍿");
    println!("===========================");
}

