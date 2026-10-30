// Official Solution for Task 16: Exam GPA & Score Averaging (Type Casting with as)

fn main() {
    let math = 85;
    let science = 92;
    let history = 78;
    let english = 90;

    let total_score = math + science + history + english;
    let total_subjects = 4;

    let average: f64 = (total_score as f64) / (total_subjects as f64);

    println!("=== STUDENT SEMESTER REPORT ===");
    println!("Math:           {}", math);
    println!("Science:        {}", science);
    println!("History:        {}", history);
    println!("English:        {}", english);
    println!("Total Score:    {}", total_score);
    println!("Subjects:       {}", total_subjects);
    println!("Exact Average:  {}%", average);
    println!("===============================");
}
