# Solution Explanation: Task 59 (Slices &[T] & &str)

Let's dissect how slices provide zero-copy window views into contiguous data structures!

---

## 🔍 The Solution Code

```rust
fn sum_slice(data: &[i32]) -> i32 {
    let mut total = 0;
    for &num in data {
        total += num;
    }
    total
}

fn main() {
    let sentence = String::from("Rustacean Power");
    let word1 = &sentence[0..9];
    let word2 = &sentence[10..];

    let readings = [12, 45, 67, 89, 90, 34, 21];
    let peak = &readings[2..5];
    let energy = sum_slice(peak);
}
```

---

## 🧩 Key Takeaways

1. **String Slices (`&str`):**
   * `&sentence[0..9]` borrows the first 9 bytes without creating any new `String` on the heap.
   * `&sentence[10..]` omits the ending index to slice up to the very end of the string.
2. **Collection Slices (`&[T]`):**
   * `&readings[2..5]` borrows elements at index 2, 3, and 4 (3 elements).
   * `sum_slice(data: &[i32])` can accept fixed arrays, vectors, or sub-slices without changing a single line of code!

