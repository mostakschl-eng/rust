# Solution Explanation: Task 60 (Lifetimes 'a)

Let's dissect how lifetime annotations protect programs from dangling pointer vulnerabilities!

---

## 🔍 The Solution Code

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let sentence = String::from("The Rust Programming Language");
    let word = "Rust";

    let result = longest(sentence.as_str(), word);
    println!("Longest text: '{}'", result);

    let banner: &'static str = "SAFE SYSTEMS WITH ZERO DANGLING POINTERS";
    println!("Security Banner: {} ('static)", banner);
}
```

---

## 🧩 Key Takeaways

1. **Why `'a` is Needed Here:**
   * `longest` takes two references (`x` and `y`) and returns one of them.
   * Without annotations, Rust has no way to tell which reference will be returned at runtime, and therefore cannot tell how long the returned reference is allowed to live!
   * The `<'a>` annotation connects all three references together, enforcing that `result` cannot outlive the shorter of `sentence` and `word`.
2. **`'static` Lifetime:**
   * Text literals (`"..."`) live in the read-only data segment of the compiled program binary for its entire lifespan.
   * Their lifetime is `'static` and they never expire!


## Reading the Result Carefully

A guest pass cannot remain valid after the underlying room lease ends. A lifetime annotation records that relationship without extending the lease.

| Symbol or term | Meaning |
|---|---|
| `'a` | named lifetime |
| `<'a>` | declares it |
| `&'a str` | borrowed text valid for a |
| `'static` | program-long referent validity |

Annotations describe relationships, not runtime extensions. .len() compares bytes here, and a static literal does not make the other input static.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
