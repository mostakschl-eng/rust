// Official Solution for Task 07: Area & Perimeter Calculator (Function Return Values)

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}

fn calculate_perimeter(width: i32, height: i32) -> i32 {
    2 * (width + height)
}

fn main() {
    let width = 10;
    let height = 5;

    let area = calculate_area(width, height);
    let perimeter = calculate_perimeter(width, height);

    println!("Room Dimensions: {} meters wide by {} meters high", width, height);
    println!("Total Area:      {} square meters", area);
    println!("Total Perimeter: {} meters", perimeter);
}
