// Official Solution for Task 18: Gourmet Cafe Receipt (Precision & Formatting)

fn main() {
    let item1_name = "Espresso";
    let item1_price = 2.75;

    let item2_name = "Caramel Latte";
    let item2_price = 4.899;

    let item3_name = "Croissant";
    let item3_price = 3.20;

    let subtotal = item1_price + item2_price + item3_price;
    let tax = subtotal * 0.08;
    let total = subtotal + tax;

    println!("========================");
    println!("{:<16}{:>8}", "ITEM", "PRICE");
    println!("========================");
    println!("{:<16}${:>7.2}", item1_name, item1_price);
    println!("{:<16}${:>7.2}", item2_name, item2_price);
    println!("{:<16}${:>7.2}", item3_name, item3_price);
    println!("------------------------");
    println!("{:<16}${:>7.2}", "Subtotal:", subtotal);
    println!("{:<16}${:>7.2}", "Tax (8%):", tax);
    println!("{:<16}${:>7.2}", "TOTAL DUE:", total);
    println!("========================");
}

