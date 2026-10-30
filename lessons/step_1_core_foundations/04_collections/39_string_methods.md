# Lesson 39: String Methods & Manipulation

Now that you understand the difference between `String` and `&str`, let's explore the powerful built-in tools Rust gives you to inspect, modify, search, and format text! 🔤

---

## 1. The Real-Life Analogy: The Word Processor Toolbox 🧰

Imagine using Microsoft Word or Google Docs:
* You want to see the **byte length** of stored text (`.len()`), which differs from a word processor’s visible-character count.
* You want to **Find and Replace** words (`.replace()`).
* You want to **check if a document contains** a specific word (`.contains()`).
* You want to **strip away unwanted white spaces** at the start and end of a line (`.trim()`).

Rust has built-in methods for all of these!

---

## 2. Essential String Methods in Action

Look at this complete demonstration:

```rust
fn main() {
    let message = String::from("  Hello, Rust World!  ");

    // 1. .trim(): Remove leading and trailing spaces
    let clean = message.trim();
    println!("Trimmed:  '{}'", clean);

    // 2. .len(): Length in bytes
    println!("Byte len: {}", clean.len());

    // 3. .contains(): Check if substring exists (returns bool)
    println!("Has 'Rust'? {}", clean.contains("Rust"));
    println!("Has 'Java'? {}", clean.contains("Java"));

    // 4. .replace(): Replace target text with new text
    let exciting = clean.replace("World", "Galaxy 🚀");
    println!("Replaced: '{}'", exciting);

    // 5. .to_uppercase() and .to_lowercase()
    println!("Loud:     '{}'", clean.to_uppercase());
    println!("Whisper:  '{}'", clean.to_lowercase());

    // 6. .push() and .push_str()
    let mut greeting = String::from("Hi");
    greeting.push('!');              // Appends a single char (single quotes '')
    greeting.push_str(" Welcome!");  // Appends a &str (double quotes "")
    println!("Greeting: '{}'", greeting);
}
```

### 📋 Expected Terminal Output:
```text
Trimmed:  'Hello, Rust World!'
Byte len: 18
Has 'Rust'? true
Has 'Java'? false
Replaced: 'Hello, Rust Galaxy 🚀!'
Loud:     'HELLO, RUST WORLD!'
Whisper:  'hello, rust world!'
Greeting: 'Hi! Welcome!'
```

---

## 3. String Concatenation: The `format!` Macro

What if you want to join multiple variables into a single new string?  
Using `+` can be clunky with ownership. Instead, Rust gives you the amazing **`format!` macro**!  
It works exactly like `println!`, but instead of printing to the screen, it **returns a brand-new `String`**:

```rust
fn main() {
    let user = "Nadia";
    let score = 950;
    let rank = 1;

    // Build a formatted message:
    let banner = format!("Player: {} | Score: {} pts | Rank #{}", user, score, rank);

    println!("{}", banner);
}
```

### 📋 Expected Terminal Output:
```text
Player: Nadia | Score: 950 pts | Rank #1
```

> [!TIP]
> Always reach for `format!(...)` when assembling complex text strings from multiple variables!

---

## 4. Characters vs Bytes (UTF-8 in Rust) 🌐

Rust strings are always valid **UTF-8** (which means they natively support emojis, Bengali, Arabic, Chinese, and every world language!).

Notice this interesting detail:
```rust
let english = "A";
let emoji = "🦀";

println!("English len: {}", english.len()); // 1 byte
println!("Emoji len:   {}", emoji.len());   // 4 bytes!
```

> [!WARNING]
> `.len()` returns the number of **bytes**, not letters!  
> To count Unicode scalar values (Rust `char` values):
> ```rust
> let crab = "🦀";
> println!("Character count: {}", crab.chars().count()); // 1
> ```

---

## 5. Summary of Common String Methods

| Method | What It Does | Example |
|---|---|---|
| `s.trim()` | Trims leading and trailing whitespace | `"  hi  ".trim()` -> `"hi"` |
| `s.contains("sub")` | Returns `true` if substring found | `s.contains("Rust")` |
| `s.replace("old", "new")` | Replaces occurrences | `s.replace("cat", "dog")` |
| `s.to_uppercase()` | Converts to capital letters | `"rust".to_uppercase()` -> `"RUST"` |
| `s.to_lowercase()` | Converts to lowercase | `"RUST".to_lowercase()` -> `"rust"` |
| `s.push(c)` | Appends single character `'c'` | `s.push('?')` |
| `s.push_str("text")` | Appends string slice `"text"` | `s.push_str(" hello")` |
| `format!(...)` | Formats and returns a new `String` | `format!("Age: {}", 25)` |

---

## 6. Common Beginner Traps ⚠️

### Trap: `.push()` vs `.push_str()`
* `.push('c')` takes a single `char` in **single quotes** `' '`.
* `.push_str("text")` takes a string slice in **double quotes** `" "`.
Mixing them up will cause a compiler error!

---

## 7. Quick Recap Checklist
- [x] `.trim()` strips away surrounding whitespace.
- [x] `.contains("word")` searches for text.
- [x] `.replace("from", "to")` creates a new string with swapped words.
- [x] Use `format!("{}-{}", a, b)` to cleanly glue variables into a string.
- [x] `.len()` measures bytes; `.chars().count()` counts Unicode scalar values, which may differ from visible characters.

---

## Unicode pitfall: letters and visible characters

A `char` is a Unicode scalar value, not necessarily a whole visible character. For example, `"e\u{301}"` combines `e` and a separate accent: `.len()` is 3 bytes, `.chars().count()` is 2, yet it usually appears as one accented letter. Some emoji also combine several scalars. Counting user-perceived characters requires grapheme segmentation, beyond this lesson. `trim()` borrows a view without changing the original string; `replace()` and case conversions produce new strings.

[Official string documentation](https://doc.rust-lang.org/std/primitive.str.html#method.chars)


## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `.trim()` | Borrow text without surrounding whitespace |
| `.len()` | UTF-8 byte length |
| `.replace(a, b)` | Build a new String |
| `format!(...)` | Build formatted owned text |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_39_string_methods/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_39_string_methods/TASK.md).
* Next lesson: [Lesson 40: Key-Value Stores: HashMaps](40_hashmaps.md).
