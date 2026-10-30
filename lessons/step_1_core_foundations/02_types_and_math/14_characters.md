# Lesson 14: Single Letters & Emojis (Characters: `char`)

Welcome to Lesson 14! 

In Lesson 01, we saw full sentences like `"Hello, World!"` wrapped in double quotes.
In Rust, there is a primitive type dedicated to holding **exactly one Unicode scalar value**: the **`char`** type.

---

## 1. Single Quotes `'` vs Double Quotes `"` (The Golden Rule)

This is the rule every programmer must memorize:

* **Single Quotes `'A'`:** Only for a **single `char`** (for example, a letter, digit, or a single-scalar emoji such as `🦀`).
* **Double Quotes `"Hello"`:** For **Strings** (words, sentences, multiple characters).

If you put two letters in single quotes `'AB'`, Rust will immediately throw a compiler error!

---

## 2. Rust's `char` is 4 Bytes (Full Unicode Support)

A Rust `char` stores one Unicode scalar value. This is different from a byte and from a complete symbol as a reader sees it.

> In Rust, every `char` is **4 bytes** in size (a Unicode Scalar Value).  
> Some visible letters and emojis contain multiple scalar values. For example, `"❤️"` and `"🇧🇩"` need strings; they cannot each fit into one `char`.

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
- [x] `char` holds exactly one Unicode scalar value, not necessarily a complete visible character.
- [x] Always use single quotes `' '` for `char`.
- [x] Rust's `char` is 4 bytes; strings hold sequences of scalar values encoded as UTF-8.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/02_types_and_math/task_14_char_and_emojis/TASK.md`](../../../tasks/step_1_core_foundations/02_types_and_math/task_14_char_and_emojis/TASK.md)**.
1. Read the task requirements.
2. Build an international greeting badge using single `char` variables and emojis.
3. Run `cargo run` and see your Unicode badge!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `'A'` | A char literal: one Unicode scalar |
| `"A"` | A borrowed string literal |

Reference: [Rust char representation](https://doc.rust-lang.org/std/primitive.char.html).
