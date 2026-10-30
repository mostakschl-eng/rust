#[allow(dead_code)]
enum PaymentMethod {
    Cash,
    CreditCard(String),
    Crypto { wallet: String, coin: String },
}

fn process_payment(method: PaymentMethod) {
    match method {
        PaymentMethod::Cash => {
            println!("Payment via Cash received. Receipt printed. 💵");
        }
        PaymentMethod::CreditCard(num) => {
            let last_4 = &num[num.len() - 4..];
            println!("Processing Credit Card [****{}]... Approved! 💳", last_4);
        }
        PaymentMethod::Crypto { wallet, coin } => {
            println!("Broadcasting {} transfer to wallet {}... Confirmed on blockchain! ⛓️", coin, wallet);
        }
    }
}

fn main() {
    println!("=== CHECKOUT PAYMENT TERMINAL ===");

    println!("Payment 1:");
    process_payment(PaymentMethod::Cash);

    println!("Payment 2:");
    process_payment(PaymentMethod::CreditCard(String::from("4111222233334242")));

    println!("Payment 3:");
    process_payment(PaymentMethod::Crypto {
        wallet: String::from("0x7a89b"),
        coin: String::from("BTC"),
    });

    println!("=================================");
}

