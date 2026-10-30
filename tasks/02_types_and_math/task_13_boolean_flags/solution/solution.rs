// Official Solution for Task 13: App Security Dashboard (Boolean Flags)

fn main() {
    let is_email_verified: bool = true;
    let has_two_factor_auth: bool = true;
    let is_account_suspended: bool = false;
    let can_make_purchases: bool = !is_account_suspended;

    println!("=== SECURITY AUDIT DASHBOARD ===");
    println!("Email Verified:        {}", is_email_verified);
    println!("Two-Factor Auth:       {}", has_two_factor_auth);
    println!("Account Suspended:     {}", is_account_suspended);
    println!("Can Make Purchases:    {}", can_make_purchases);
    println!("================================");
}
