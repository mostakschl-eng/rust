#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    fn is_active(&self) -> bool {
        *self == OrderStatus::Pending || *self == OrderStatus::Shipped
    }
}

fn main() {
    println!("=== E-COMMERCE ORDER TRACKER ===");

    let order_a = OrderStatus::Pending;
    let order_b = OrderStatus::Delivered;
    let order_c = OrderStatus::Cancelled;

    println!("Order A status: {:?} (Active: {})", order_a, order_a.is_active());
    println!("Order B status: {:?} (Active: {})", order_b, order_b.is_active());
    println!("Order C status: {:?} (Active: {})", order_c, order_c.is_active());

    println!("================================");
}
