# Lesson 18: Formatted Output & Number Precision (`{:.2}`)

Welcome to Lesson 18! 

In Lesson 12, when we calculated gas prices, the output had three or four decimals: `$45.625`.
In the real world, cash registers only show **2 decimal places**: `$45.63`!

Today, we will learn how to make our terminal outputs look like **professional receipts and dashboards** using Rust's formatting specifiers!

---

## 1. Controlling Decimal Places: `{:.2}`

Look at the magic inside the curly braces: `{:.2}`

* The period `.` means: *"Look after the decimal point."*
* The number `2` means: *"Show exactly 2 digits, and round properly!"*

```rust
fn main() {
    let pi = 3.14159265359;
    let gas_total = 45.625;

    println!("Full Pi:      {}", pi);
    println!("Rounded Pi:   {:.2}", pi);    // 3.14
    println!("Rounded Gas:  ${:.2}", gas_total); // $45.63 (rounds up!)
}
```

### The Output on Screen:
```text
Full Pi:      3.14159265359
Rounded Pi:   3.14
Rounded Gas:  $45.63
```

---

## 2. Named Arguments & Reusing Values

Did you know you can name your placeholders or repeat values without typing them twice?

```rust
fn main() {
    // 1. Positional arguments: {0} and {1}
    println!("{0} loves {1}, and {1} loves {0}!", "Rahim", "Rust");

    // 2. Named arguments:
    println!("{person} scored {points} points in {game}!",
        person = "Rahim",
        points = 95,
        game = "Rust Quest"
    );
}
```

### The Output on Screen:
```text
Rahim loves Rust, and Rust loves Rahim!
Rahim scored 95 points in Rust Quest!
```

---

## 3. Formatting Alignment: Left, Right, Center

When building tables, you want text and numbers to line up in straight vertical columns:

| Format Code | Meaning | Example Output |
|---|---|---|
| `{:<10}` | Left-aligned in 10 spaces | `"Apple     "` |
| `{:>10}` | Right-aligned in 10 spaces | `"     Apple"` |
| `{:^10}` | Centered in 10 spaces | `"  Apple   "` |

```rust
fn main() {
    println!("|{:<10}|{:>8}|", "Item", "Price");
    println!("|{:<10}|{:>8.2}|", "Coffee", 3.5);
    println!("|{:<10}|{:>8.2}|", "Cheesecake", 12.0);
}
```

### The Output on Screen:
```text
|Item      |   Price|
|Coffee    |    3.50|
|Cheesecake|   12.00|
```

---

## 4. Secret Hacker Trick: Printing in Binary (`{:b}`) and Hex (`{:x}`)

Want to see what numbers look like in computer bits?
```rust
fn main() {
    let number = 42;
    println!("Decimal:     {}", number);
    println!("Binary bits: {:b}", number); // 101010
    println!("Hexadecimal: {:x}", number); // 2a
}
```

---

## 5. Summary Checklist
- [x] `{:.2}` rounds a float to 2 decimal places.
- [x] `{0}` and `{1}` let you reuse arguments.
- [x] `{:<10}` and `{:>10}` align text into beautiful vertical columns.
- [x] `{:b}` displays numbers in binary ones and zeros.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_18_number_formatting/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_18_number_formatting/TASK.md)**.
1. Read the task requirements.
2. Build an invoice table with neatly aligned columns and 2-decimal rounded currency.
3. Run `cargo run` and admire your formatted invoice!
