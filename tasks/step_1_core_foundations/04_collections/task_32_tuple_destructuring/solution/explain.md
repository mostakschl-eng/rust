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
The function returns one tuple value containing three results: `(sum, avg, max)`. The positions in the returned tuple must match the declared return type.

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

## Everyday picture and symbols

Think of unpacking an exam report into total, average, and best score. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `let (a, b) = pair` | Destructure by position |
| `_` | Ignore this position |
| `as f64` | Convert the sum before division |

## Common pitfall

Divide a floating-point sum by `3.0`; integer division would lose the fraction. Scores in this exercise are 0–100, so their sum fits in i32.

## Expected output

```text
=== EXAM BATCH 1 RESULTS ===
Total Points:  255
Average Score: 85.00
Highest Score: 92

=== EXAM BATCH 2 RESULTS ===
Batch 2 Top Score: 100
============================
```
