// Official Solution for Task 15: Egg Farm Packaging Calculator (Arithmetic & %)

fn main() {
    let total_eggs = 58;
    let carton_size = 12;

    let full_cartons = total_eggs / carton_size;
    let leftover_eggs = total_eggs % carton_size;

    println!("=== SUNNYVALE EGG FARM PACKAGING ===");
    println!("Total Eggs Collected: {}", total_eggs);
    println!("Carton Size:          {} eggs per carton", carton_size);
    println!("Full Cartons Packed:  {} cartons ({} eggs total)", full_cartons, full_cartons * carton_size);
    println!("Leftover Eggs:        {} loose eggs", leftover_eggs);
    println!("====================================");
}

