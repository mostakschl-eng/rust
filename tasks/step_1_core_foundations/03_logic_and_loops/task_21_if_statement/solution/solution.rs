// Official Solution for Task 21: ATM Cash Dispenser (The `if` Statement)

fn main() {
    let account_balance = 500;
    let withdraw_amount = 200;

    println!("ATM: Processing withdrawal request of ${}...", withdraw_amount);

    if account_balance >= withdraw_amount {
        let remaining = account_balance - withdraw_amount;
        println!("Success! Dispensing ${} cash...", withdraw_amount);
        println!("Remaining balance: ${}", remaining);
    }

    println!("ATM: Thank you for banking with us.");
}

