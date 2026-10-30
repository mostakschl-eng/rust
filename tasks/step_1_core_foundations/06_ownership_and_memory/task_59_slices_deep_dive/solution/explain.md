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


## Reading the Result Carefully

A window reveals part of a wall without cutting out bricks. A slice similarly views part of existing storage.

| Symbol or term | Meaning |
|---|---|
| `&[T]` | borrowed element slice |
| `&str` | borrowed UTF-8 text |
| `a..b` | includes a, excludes b |

String ranges are byte ranges and must land on UTF-8 character boundaries. For example, &"é"[0..1] panics. Out-of-bounds and reversed ranges also panic; .get(range) returns Option instead.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
