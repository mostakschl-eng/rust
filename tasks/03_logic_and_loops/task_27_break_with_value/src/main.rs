// Task 27: Savings Goal Investment Tracker (`break value;`)
//
// INSTRUCTIONS:
// 1. Declare balance = 200, years = 0.
// 2. Compute years_taken using `let years_taken = loop { ... break years; };`
// 3. Print the results and test with `cargo run`.

fn main() {
    println!("=== SAVINGS GOAL CALCULATOR ===");
    println!("Initial Deposit: $200");
    println!("Target Goal:     $1000");

    let mut balance = 200;
    let mut years = 0;

    // TODO: Write your loop returning years with `break years;` below:
    let years_taken = loop {
        years += 1;
        balance += 150;
        if balance >= 1000 {
            break years;
        }
    };

    println!("Calculating annual deposits of $150...");
    println!("Goal Reached! Target surpassed in {} years!", years_taken);
    println!("Final Balance:   ${}", balance);
    println!("===============================");
}

