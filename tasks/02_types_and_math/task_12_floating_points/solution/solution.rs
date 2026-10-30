// Official Solution for Task 12: Gas Station Fuel Pump Calculator (Floating-Point f64)

fn main() {
    let price_per_gallon: f64 = 3.85;
    let gallons_pumped: f64 = 12.5;

    let fuel_cost: f64 = gallons_pumped * price_per_gallon;
    let car_wash_discount: f64 = 2.50;
    let final_total: f64 = fuel_cost - car_wash_discount;

    println!("=== METRO GAS STATION ===");
    println!("Price Per Gallon: ${}", price_per_gallon);
    println!("Gallons Pumped:   {}", gallons_pumped);
    println!("Fuel Subtotal:    ${}", fuel_cost);
    println!("Car Wash Coupon:  -${}", car_wash_discount);
    println!("Final Total Due:  ${}", final_total);
    println!("=========================");
}

