// Official Solution for Task 29: Push-Up Repetition Counter (`for` Loops & Ranges)

fn main() {
    println!("COACH: Set 1 — Starting 10 push-ups! Let's go! 💪");

    for rep in 1..=10 {
        println!("Rep {}: Good form!", rep);

        if rep == 5 {
            println!("Halfway there! Keep pushing! 🔥");
        }
    }

    println!("COACH: Set Complete! Great work! Take a 30-second rest. 🏆");
}

