# Lesson 11: Whole Numbers (Integers: Signed `i` vs Unsigned `u`)

Welcome to **Level 2: Basic Types & Arithmetic**! 

In Level 1, you learned how to write functions and create variable boxes.
Now, we are going to dive deep into **what kinds of data computers can store**. 

We start with the most common data type in the universe: **Whole Numbers (Integers)**.

---

## 1. What is an Integer?

An **integer** is any number that does NOT have a decimal fraction.
* `1`, `42`, `1000`, `0` are all integers.
* `3.14` is **not** an integer (it has a decimal point).

---

## 2. Signed (`i`) vs Unsigned (`u`): The Fork in the Road

In Rust, every integer belongs to one of two families:

### Family 1: Signed Integers (Starts with `i`)
* The letter `i` stands for **Integer**.
* Can hold **both positive numbers and negative numbers** (`-50`, `0`, `+50`).
* Real-life example: Temperature in Celsius (it can be `+35` in summer or `-10` in winter).

### Family 2: Unsigned Integers (Starts with `u`)
* The letter `u` stands for **Unsigned** (*no minus sign allowed*).
* Can **ONLY hold positive numbers or zero** (`0`, `1`, `2`, `100...`).
* Real-life example: Your age, the number of pages in a book, or your YouTube subscriber count can **never be negative**!

---

## 3. The Bit Sizes: Small Boxes vs Giant Vaults

Computers store numbers in memory bits (ones and zeros). 
Rust lets you choose how big of a box you need:

| Type | How many bits? | Range in Plain English | Good For |
|---|---|---|---|
| `u8` | 8 bits (1 byte) | `0` to `255` | RGB color values, tiny counters |
| `i8` | 8 bits (1 byte) | `-128` to `127` | Small positive/negative offsets |
| `u16` / `i16` | 16 bits (2 bytes) | `0` to `65,535` / `-32,768` to `32,767` | Port numbers, game sprite IDs |
| **`i32`** | 32 bits (4 bytes) | **-2 Billion to +2 Billion** | **The Rust Default!** Fast & covers 95% of normal numbers |
| `u32` | 32 bits (4 bytes) | `0` to `4.2 Billion` | Positive counts, phone numbers |
| `i64` / `u64` | 64 bits (8 bytes) | Huge astronomical numbers | World population, bank account balances in cents |
| `usize` | Pointer size | 64-bit on modern computers | Indexing lists and counting memory elements |

---

## 4. Code Example: Choosing the Right Integer

```rust
fn main() {
    let temperature: i8 = -5;          // Can be negative!
    let player_age: u8 = 25;           // Cannot be negative!
    let website_visitors: u32 = 150_000; // Big positive count
    let national_debt: i64 = 34_000_000_000_000; // Giant 64-bit number

    println!("Winter Temperature: {} C", temperature);
    println!("Player Age:         {} years", player_age);
    println!("Monthly Visitors:   {}", website_visitors);
    println!("National Debt:      ${}", national_debt);
}
```

> **Pro Tip:** In Rust, you can put underscores `_` in large numbers (like `150_000` or `1_000_000`) to make them easy for human eyes to read! Rust ignores the underscores.

### The Output on Screen:
```text
Winter Temperature: -5 C
Player Age:         25 years
Monthly Visitors:   150000
National Debt:      $34000000000000
```

---

## 5. Common Beginner Traps

### ❌ Trap 1: Putting a Negative Number in an Unsigned `u` Box
```rust
let age: u8 = -10; // ❌ ERROR!
```
* **What Rust says:** `error: cannot apply unary operator '-' to type 'u8'`.  
  Rust protects you: *"You promised this box was unsigned (only positive), but you gave me -10!"*

---

## 6. Summary Checklist
- [x] Integers are whole numbers without decimals.
- [x] `i` = signed (can be negative).
- [x] `u` = unsigned (only positive and zero).
- [x] `i32` is Rust's favorite default whole number.
- [x] Use `_` to make big numbers readable (e.g. `1_000_000`).

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_11_integer_sizes/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_11_integer_sizes/TASK.md)**.
1. Read the task requirements.
2. Select the perfect integer types (`i8`, `u8`, `u32`, `i64`) for real-world stats.
3. Run `cargo run` and test your integer knowledge!
