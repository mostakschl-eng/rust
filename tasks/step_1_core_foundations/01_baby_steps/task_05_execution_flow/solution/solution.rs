// Official Solution for Task 05: Rocket Launch Sequence (Execution Flow)

fn check_fuel() {
    println!("Fuel tanks at 100% capacity.");
}

fn check_engines() {
    println!("All engines online and ready.");
}

fn launch() {
    println!("--- Pre-flight checks starting ---");
    check_fuel();
    check_engines();
    println!("--- Liftoff in 3... 2... 1... BLASTOFF! 🚀 ---");
}

fn main() {
    println!("Mission Control: Initiating Launch Protocol");
    launch();
}

