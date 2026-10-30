# Lesson 50: Concise Matching: `if let` and `let else`

Congratulations on reaching **Lesson 50**! 🎉

In Lessons 48 and 49, we learned about `match`.  
`match` is amazing when you have 4 or 5 different scenarios to handle.  
However, sometimes you find yourself writing code like this:

```rust
let config_max: Option<u32> = Some(100);

match config_max {
    Some(max) => println!("The maximum limit is: {}", max),
    None => (), // Ignore everything else... boilerplate!
}
```
Having to write `_ => ()` or `None => ()` just to satisfy the compiler when you **only care about ONE case** feels tedious.

Rust gives us two elegant shortcuts: **`if let`** and **`let else`**! ⚡

---

## 1. The Real-Life Analogy: The Express Checkout Lane 🛒

* **Full `match`:** You are an inspector opening a shipping container, cataloging all 50 different item categories one by one.
* **`if let`:** You are looking for a single specific item: *"If the delivery is a golden ticket, celebrate! Otherwise, ignore it."*
* **`let else`:** You are an airport security gate: *"Extract the passenger's boarding pass; if they don't have one, kick them out of the line immediately!"*

---

## 2. The `if let` Expression

`if let` takes a pattern and an expression separated by an equal sign `=`:

```rust
fn main() {
    let mystery_box: Option<i32> = Some(42);

    // If mystery_box matches the Some(number) pattern, extract 'number' and run the block!
    if let Some(number) = mystery_box {
        println!("You found the secret number: {}!", number);
    } else {
        println!("The box was empty!");
    }
}
```

### 📋 Expected Terminal Output:
```text
You found the secret number: 42!
```

Look how clean that is! No `_ => ()` boilerplate needed.

---

## 3. The `let else` Guard Statement (Rust 1.65+ Superpower!) 🛡️

What if you are inside a function and you want to extract a value, but if the value is missing, you want to **return early / exit immediately**?

In the past, code suffered from nested indentation ("Pyramid of Doom").  
**`let else`** solves this permanently!

```rust
fn process_username(raw_input: Option<String>) {
    // If raw_input is Some(name), unwrap it into 'name'.
    // ELSE: execute the divergence block and return early!
    let Some(name) = raw_input else {
        println!("⚠️ Error: No username provided. Aborting process.");
        return;
    };

    // 'name' is now a normal, unpacked String available for the rest of the function!
    println!("Processing user: '{}' (Length: {} bytes)", name, name.len());
}

fn main() {
    process_username(Some(String::from("AdaLovelace")));
    process_username(None);
}
```

### 📋 Expected Terminal Output:
```text
Processing user: 'AdaLovelace' (Length: 11 bytes)
⚠️ Error: No username provided. Aborting process.
```

> [!IMPORTANT]
> The `else` block of a `let else` statement **must diverge**!  
> That means it must exit the current scope using `return`, `break`, `continue`, or `panic!`.

---

## 4. Summary: Which Matching Tool When?

| Tool | When to Use It |
|---|---|
| **`match`** | When you want to handle all possibilities explicitly, even for two variants |
| **`if let`** | When you care about only ONE pattern and want to ignore the rest |
| **`if let ... else`** | When you care about one pattern and have a simple fallback |
| **`let else`** | When extracting data into the local scope and early-exiting on failure (flattening code!) |

---

## 5. Quick Recap Checklist
- [x] `if let Pattern = value` handles single pattern matching concisely.
- [x] An optional `else` block handles all other non-matching cases.
- [x] `let Pattern = value else { ... }` extracts variables into local scope or exits early.
- [x] The `else` block in `let else` must diverge (`return`, `break`, or `panic!`).

---

## Pitfall: presence is not validation

`Some(name)` only means a String exists: it may still be empty or invalid for your application. A `let else` failure block must not continue into code that assumes the binding exists. In a function use `return`; inside a loop `break` or `continue` may be appropriate. The statement ends with `;`.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `if let Some(x) = value` | Handle one successful pattern |
| `let Some(x) = value else` | Bind x or leave on failure |
| `return;` | Leave the current function |
| `};` | Finish a let-else statement |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_50_if_let_and_let_else/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_50_if_let_and_let_else/TASK.md).
* You've completed Level 5! Next up: [Level 6: Rust's Sacred Core — Ownership & Memory](../06_ownership_and_memory/51_stack_vs_heap.md).

## Continue Your Study Tracker

After finishing this level's numbered tasks, build [checkpoint 5](../../../tasks/step_1_core_foundations/05_structs_and_enums/project_05_session_models/TASK.md). Bring forward your own previous checkpoint; keep its working version separate.
