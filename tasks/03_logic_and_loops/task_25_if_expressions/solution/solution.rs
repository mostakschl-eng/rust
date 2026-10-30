// Official Solution for Task 25: E-Commerce Free Shipping Calculator (`if` Expression)

fn main() {
    let order_total = 65.50;

    let shipping_fee: f64 = if order_total >= 50.0 {
        0.00
    } else {
        7.99
    };

    let final_bill = order_total + shipping_fee;

    println!("=== ONLINE STORE CHECKOUT ===");
    println!("Order Subtotal:   ${:.2}", order_total);
    if shipping_fee == 0.0 {
        println!("Free Shipping Qualified!");
    }
    println!("Shipping Fee:     ${:.2}", shipping_fee);
    println!("Final Total:      ${:.2}", final_bill);
    println!("=============================");
}

