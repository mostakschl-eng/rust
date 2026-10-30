# Solution Explanation: Task 32 (Tuple Destructuring)

Let's dissect how returning tuples and destructuring makes function outputs clean and expressive!

---

## 🔍 The Solution Code

```rust
fn calculate_stats(s1: i32, s2: i32, s3: i32) -> (i32, f64, i32) {
    let sum = s1 + s2 + s3;
    let avg = sum as f64 / 3.0;
    let mut max = s1;
    if s2 > max { max = s2; }
    if s3 > max { max = s3; }
    (sum, avg, max)
}

fn main() {
    println!("=== EXAM BATCH 1 RESULTS ===");

    let (total, average, highest) = calculate_stats(85, 92, 78);

    println!("Total Points:  {}", total);
    println!("Average Score: {:.2}", average);
    println!("Highest Score: {}", highest);

    println!("\n=== EXAM BATCH 2 RESULTS ===");

    let (_, _, top_score) = calculate_stats(90, 95, 100);

    println!("Batch 2 Top Score: {}", top_score);
    println!("============================");
}
```

---

## 🧩 Key Concepts Explained

### 1. Returning Multiple Values via Tuples:
```rust
fn calculate_stats(...) -> (i32, f64, i32)
```
In many older languages, if you need a function to return 3 numbers, you have to create a custom class or pass pointers. In Rust, you simply wrap them in `(sum, avg, max)` and return them!

### 2. Full Destructuring:
```rust
let (total, average, highest) = calculate_stats(85, 92, 78);
```
Here, Rust automatically maps:
* `total` <- `sum`
* `average` <- `avg`
* `highest` <- `max`

### 3. Selective Destructuring with Wildcards (`_`):
```rust
let (_, _, top_score) = calculate_stats(90, 95, 100);
```
We told the compiler: *"Throw away items 0 and 1; bind only item 2 to `top_score`"*. This prevents unused variable warnings!

