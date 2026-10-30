# Lesson 38: Text in Memory: `String` vs `&str`

Almost every beginner in Rust eventually asks:
> *"Why does Rust have two different string types: `String` and `&str`? Why can't it just be `string` like Python or JavaScript?"*

Today, we solve this mystery once and for all with a crystal-clear real-life analogy! 🪧 vs 📖

---

## 1. The Real-Life Analogy: The Billboard vs The Spiral Notebook

### The Billboard (`&str` — String Slice):
* Imagine you look up at a metal billboard on the highway that says: **"WELCOME TO RUST CITY"**.
* The letters are permanently painted onto the metal.
* You cannot easily erase a letter, stretch the metal, or add 10 more words to it.
* You are just **looking at** (referencing) fixed text.

### The Spiral Notebook (`String` — Owned String):
* Imagine you are holding a spiral-bound notebook in your hands.
* You write "Hello" on page 1.
* A minute later, you write " World!" right next to it.
* If you run out of room, you can turn the page or glue extra paper inside!
* **You own the notebook**, and it can **grow and change** whenever you want.

---

## 2. Comparing `&str` vs `String` Side-by-Side

| Feature | `&str` (String Slice) | `String` (Owned String) |
|---|---|---|
| **What is it?** | A view / reference to existing text | An owned, growable container of text |
| **Memory Location** | A borrowed view of UTF-8 text in a literal, String, or other valid storage | UTF-8 bytes on the heap; controller stored with its owner |
| **Growable?** | ❌ No (Fixed size) | ✅ Yes (Can expand with `push_str`) |
| **How to create** | Double quotes: `"Hello"` | `String::from("Hello")` or `"Hello".to_string()` |
| **Analogy** | Looking at a billboard | Writing in your own notebook |

---

## 3. How to Create and Convert Between Them

Look at how they interact:

```rust
fn main() {
    // 1. &str: A fixed string literal (billboard)
    let greeting_slice: &str = "Hello";

    // 2. Convert &str into an owned, growable String (notebook)
    let mut greeting_owned: String = String::from(greeting_slice);

    // 3. We can grow an owned String!
    greeting_owned.push_str(", Rustacean! 🦀");

    println!("Slice: {}", greeting_slice);
    println!("Owned: {}", greeting_owned);

    // 4. Convert owned String back to a &str view using &
    let view_slice: &str = &greeting_owned;
    println!("View:  {}", view_slice);
}
```

### 📋 Expected Terminal Output:
```text
Slice: Hello
Owned: Hello, Rustacean! 🦀
View:  Hello, Rustacean! 🦀
```

---

## 4. Why Does Rust Do This? (Zero-Cost Safety!)

Borrowing a view lets Rust inspect existing text without copying its bytes. Other languages have their own string representations; allocation costs depend on the operation.

In Rust:
1. If you write `"Ready"`, Rust embeds `"Ready"` directly into the compiled executable file. Reading it costs **zero memory allocation**!
2. If you need dynamic user text (from a keyboard, file, or network), Rust gives you `String`, which allocates heap memory only when you explicitly need it.

---

## 5. Which One Should You Use in Function Parameters?

When writing a function that accepts text, **prefer `&str` when the function only needs to read text**:

```rust
// ✅ Best practice: Can accept BOTH &str literals AND &String!
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let literal: &str = "Alice";
    let owned: String = String::from("Bob");

    greet(literal); // Works!
    greet(&owned);  // Works automatically thanks to Rust deref coercion!
}
```

> [!TIP]
> If your function takes `&str`, callers can pass either string literals `"text"` OR references to owned strings `&my_string`. Use `&mut String` to edit an existing owned string, or `String` to take ownership when that is the function’s purpose.

---

## 6. Common Beginner Traps ⚠️

### Trap: Trying to Modify a `&str`
```rust
let mut message: &str = "Hello";
message.push_str(" World"); // ❌ ERROR: no method named `push_str` found for type `&str`
```
**Fix:** Convert it to an owned `String` first:
```rust
let mut message = String::from("Hello");
message.push_str(" World"); // ✅ Works!
```

---

## 7. Quick Recap Checklist
- [x] `"hello"` is a string slice of type `&str` (fixed, read-only view).
- [x] `String::from("hello")` creates an owned, growable `String` on the heap.
- [x] Use `push_str(&str)` to append text to an owned `String`.
- [x] In function parameters, prefer `&str` for maximum flexibility.

---

## Scope of the analogy

A `&str` is a view, not necessarily permanent text. It may borrow part of a local `String` and cannot outlive its owner. `let mut view: &str` lets you point the variable at different text; it does not make the borrowed text editable through that shared reference. Both string types hold valid UTF-8. Safe string slicing uses byte positions at UTF-8 boundaries; `s[0]` is not supported.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `String` | Owned growable UTF-8 text |
| `&str` | Shared view of existing UTF-8 text |
| `&badge_code` | Borrow the owned String |
| `.push('-')` | Append one char |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_38_string_vs_str/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_38_string_vs_str/TASK.md).
* Next lesson: [Lesson 39: String Methods & Manipulation](39_string_methods.md).
