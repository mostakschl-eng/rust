#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TaskItem {
    title: String,
    priority: u8,
}

fn main() {
    println!("=== DOCUMENT DEEP COPY ENGINE ===");

    let mut original_doc = String::from("Draft v1");
    let mut cloned_doc = original_doc.clone();

    original_doc.push_str(" (Archived Copy)");
    cloned_doc.push_str(" (Revised by Editor)");

    println!("Original: {}", original_doc);
    println!("Cloned:   {}", cloned_doc);

    println!("\n=== CLONING CUSTOM STRUCT ===");

    let task1 = TaskItem {
        title: String::from("Fix Server Crash"),
        priority: 5,
    };
    let mut task2 = task1.clone();
    task2.priority = 1;

    println!("Task 1: {:?}", task1);
    println!("Task 2: {:?}", task2);

    println!("=================================");
}
