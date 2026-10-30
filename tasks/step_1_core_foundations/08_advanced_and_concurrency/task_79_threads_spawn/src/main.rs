use std::thread;
use std::time::Duration;

fn main() {
    println!("=== PARALLEL WORKER THREAD ===");

    let dataset = vec![10, 20, 30, 40, 50];

    println!("[Main] Dispatching background task...");

    // TODO 1: Spawn thread with move closure returning (sum, max)
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        println!("[Worker] Calculations complete!");

        let sum: i32 = dataset.iter().sum();
        let max: i32 = *dataset.iter().max().unwrap_or(&0);

        (sum, max)
    });

    println!("[Main] Waiting for worker thread to complete...");

    // TODO 2: Join the thread and retrieve the result
    let (sum, max) = handle.join().unwrap();

    println!("Sum: {}, Max: {}", sum, max);

    println!("==============================");
}

