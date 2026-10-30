struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn display(&self) {
        println!("Account Holder: {} | Balance: ${:.2}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: f64) {
        if !amount.is_finite() || amount <= 0.0 || !(self.balance + amount).is_finite() {
            println!("Deposit rejected: amount must be positive and keep balance finite.");
            return;
        }
        self.balance += amount;
        println!("Deposited ${:.2} successfully.", amount);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount.is_finite() && amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

fn main() {
    println!("=== SMART ATM BANKING SYSTEM ===");

    let mut account = BankAccount {
        owner: String::from("Tariq Ali"),
        balance: 200.00,
    };

    account.display();

    account.deposit(150.00);

    if account.withdraw(50.00) {
        println!("Withdrew $50.00 successfully.");
    } else {
        println!("⚠️ Overdraft Alert: Insufficient funds for $50.00!");
    }

    if account.withdraw(500.00) {
        println!("Withdrew $500.00 successfully.");
    } else {
        println!("⚠️ Overdraft Alert: Insufficient funds for $500.00!");
    }

    account.display();

    println!("================================");
}

