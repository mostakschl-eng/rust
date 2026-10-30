// Official Solution for Task 06: Product Receipt Line Printer (Function Inputs)

fn print_item(item_name: &str, quantity: i32, price: i32) {
    println!("Item: {:<10} | Qty: {} | Price: ${}", item_name, quantity, price);
}

fn main() {
    println!("=== GROCERY RECEIPT ===");
    print_item("Laptop", 1, 900);
    print_item("Headphones", 2, 50);
    print_item("Coffee Mug", 3, 12);
    println!("=======================");
}
