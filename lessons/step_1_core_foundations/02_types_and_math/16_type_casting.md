# Lesson 16: Type Casting & Explicit Conversion (The `as` Keyword)

Welcome to Lesson 16! 

In Lesson 12, we learned that Rust **never mixes different types automatically**.
If you have an integer (`i32`) and you want to divide it or multiply it with a decimal (`f64`), what do you do?

You perform an **Explicit Type Cast** using the **`as`** keyword!

---

## 1. The Real-Life Analogy: The Currency Exchange Booth

Imagine you land in Tokyo with US Dollars in your wallet. 
You cannot hand US dollar bills to a Japanese subway vending machine. It will reject the bills.

You have to go to an official **Exchange Booth** and say:
> *"Please convert my **10 Dollars as Japanese Yen**!"*

In Rust, the **`as`** keyword is your official conversion booth!

---

## 2. Converting an Integer to a Float (`as f64`)

Imagine 3 students take an exam and their total score is `280` points.
You want to calculate an approximate average score with decimals:

```rust
fn main() {
    let total_score: i32 = 280;
    let student_count: i32 = 3;

    // ❌ Without 'as', 280 / 3 gives 93 (loses the fraction .333!)
    // ✅ With 'as f64', we convert the integer into a float:
    let average: f64 = (total_score as f64) / (student_count as f64);

    println!("Total Points:  {}", total_score);
    println!("Student Count: {}", student_count);
    println!("Average (2 dp): {:.2}", average); // 93.33!
}
```

### The Output on Screen:
```text
Total Points:  280
Student Count: 3
Average (2 dp): 93.33
```

Notice: `total_score as f64` converts the whole number `280` into the decimal `280.0`. Now float division works smoothly!

---

## 3. Converting a Float to an Integer (`as i32`)

What happens if you convert a decimal number into an integer?
Rust **chops off (truncates) the decimal part**:

```rust
fn main() {
    let price: f64 = 19.99;
    let dollars: i32 = price as i32; // Chops off .99!

    println!("Original Price: ${}", price);
    println!("Whole Dollars:  ${}", dollars);
}
```

### The Output on Screen:
```text
Original Price: $19.99
Whole Dollars:  $19
```
*(Notice: Rust does not round `19.99` up to `20`. It simply discards the fraction `.99`!)*

---

## 4. Common Beginner Traps

### ❌ Trap 1: Integer Overflow When Casting
Be careful when converting a large number into a tiny box:
```rust
let big_number: i32 = 300;
let tiny_box: u8 = big_number as u8; // ❌ DANGER!
println!("{}", tiny_box); // Prints 44, NOT 300!
```
* **Why:** A `u8` can only hold numbers up to `255`. When you try to force `300` into it, it wraps around (`300 - 256 = 44`)!
* **Rule:** Only cast into smaller types when you are 100% sure the number fits!

---

## 5. Summary Checklist
- [x] Use `value as NewType` to explicitly convert data.
- [x] `integer as f64` turns a whole number into a decimal.
- [x] `float as i32` chops off the decimal fraction.
- [x] `as` makes a cast explicit, but it can still silently discard information. Check the source and destination ranges.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/02_types_and_math/task_16_type_casting/TASK.md`](../../../tasks/step_1_core_foundations/02_types_and_math/task_16_type_casting/TASK.md)**.
1. Read the task requirements.
2. Build an exam average score calculator that converts integers to `f64` using `as f64`.
3. Run `cargo run` and test your type casting!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `as` | Explicit numeric cast; may lose information |
| `f64` | 64-bit binary floating-point type |

## Common pitfalls and limits

Casting is not validation. Narrow integer casts can discard bits, and large integers may lose precision as floats. Float-to-integer casts truncate toward zero, clamp out-of-range values to the target limits, and map NaN to zero. Checked conversion is a later topic.

The currency-booth analogy only means changing a representation. A numeric cast does not apply a currency exchange rate. See the [Rust Reference on numeric casts](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions).
