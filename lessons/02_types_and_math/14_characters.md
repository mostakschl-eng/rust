# Lesson 14: Single Letters & Emojis (Characters: `char`)

Welcome to Lesson 14! 

In Lesson 01, we saw full sentences like `"Hello, World!"` wrapped in double quotes.
In Rust, there is a special, ultra-fast primitive type dedicated to holding **exactly one single character**: the **`char`** type.

---

## 1. Single Quotes `'` vs Double Quotes `"` (The Golden Rule)

This is the rule every programmer must memorize:

* **Single Quotes `'A'`:** Only for a **single `char`** (one single letter, digit, or emoji).
* **Double Quotes `"Hello"`:** For **Strings** (words, sentences, multiple characters).

If you put two letters in single quotes `'AB'`, Rust will immediately throw a compiler error!

---

## 2. Rust's `char` is 4 Bytes (Full Unicode Support)

In older programming languages (like C), a character was only 1 byte (ASCII), which could only hold English letters.
Rust was built for the modern global internet!

> In Rust, every `char` is **4 bytes** in size (a Unicode Scalar Value).  
> This means a Rust `char` can hold **any character in any language on Earth, plus emojis**!

Look at this amazing code:

```rust
fn main() {
    let letter: char = 'A';
    let symbol: char = '$';
    let bangla_letter: char = 'ক';
    let japanese_letter: char = '日';
    let favorite_emoji: char = '🦀'; // The Rust Crab mascot!

    println!("English:  {}", letter);
    println!("Symbol:   {}", symbol);
    println!("Bengali:  {}", bangla_letter);
    println!("Japanese: {}", japanese_letter);
    println!("Emoji:    {}", favorite_emoji);
}
```

### The Output on Screen:
```text
English:  A
Symbol:   $
Bengali:  ক
Japanese: 日
Emoji:    🦀
```

---

## 3. Common Beginner Traps

### ❌ Trap 1: Using Double Quotes for a `char`
```rust
let grade: char = "A"; // ❌ ERROR!
```
* **What Rust says:** `error: mismatched types: expected 'char', found '&str'`.
* **Fix:** Use single quotes: `let grade: char = 'A';`.

### ❌ Trap 2: Putting More Than One Character in Single Quotes
```rust
let letters: char = 'ABC'; // ❌ ERROR!
```
* **What Rust says:** `error: character literal may only contain one codepoint`.

---

## 4. Summary Checklist
- [x] `char` holds exactly one single character.
- [x] Always use single quotes `' '` for `char`.
- [x] Rust's `char` is 4 bytes and supports any global language and emojis!

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_14_char_and_emojis/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_14_char_and_emojis/TASK.md)**.
1. Read the task requirements.
2. Build an international greeting badge using single `char` variables and emojis.
3. Run `cargo run` and see your Unicode badge!
