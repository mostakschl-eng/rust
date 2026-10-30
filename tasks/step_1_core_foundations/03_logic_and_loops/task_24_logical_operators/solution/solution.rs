// Official Solution for Task 24: Flight Boarding Pass & ID Verifier (Logical `&&` and `||`)

fn main() {
    let has_ticket = true;
    let has_passport = false;
    let has_national_id = true;

    println!("=== AIRPORT BOARDING GATE ===");
    println!("Ticket Verified:      {}", has_ticket);
    println!("Passport Available:   {}", has_passport);
    println!("National ID Card:     {}", has_national_id);

    if has_ticket && (has_passport || has_national_id) {
        println!("Gate Open: Welcome aboard flight 707! ✈️");
    } else {
        println!("Gate Closed: Missing valid ID or ticket! 🛑");
    }

    println!("=============================");
}

