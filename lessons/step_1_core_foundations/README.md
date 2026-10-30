# Step 1: Core Rust Foundations (80 Lessons: Child to Adult)

Welcome to **Step 1** of your Rust journey! This stage introduces core syntax, ownership, error handling, and concurrency across eight levels. It is a foundation; Rust has more language features and libraries to explore later.

---

## 📚 Curriculum Breakdown

| Level | Topic | Description | Lessons |
|:---:|---|---|:---:|
| **01** | [`01_baby_steps/`](01_baby_steps/) | Instructions, `fn main`, `println!`, functions, variables, `mut` | 01 – 10 |
| **02** | [`02_types_and_math/`](02_types_and_math/) | Integers (`i32`), floats (`f64`), bool, char, casting, formatting | 11 – 20 |
| **03** | [`03_logic_and_loops/`](03_logic_and_loops/) | `if`/`else`, `loop`, `while`, `for`, ranges, break with values | 21 – 30 |
| **04** | [`04_collections/`](04_collections/) | Tuples, arrays, `Vec<T>`, slices, `String` vs `&str`, `HashMap` | 31 – 40 |
| **05** | [`05_structs_and_enums/`](05_structs_and_enums/) | Structs, methods (`impl`), enums, optionals, `match` patterns | 41 – 50 |
| **06** | [`06_ownership_and_memory/`](06_ownership_and_memory/) | Stack vs Heap, 3 Ownership rules, Moves, Borrowing (`&`, `&mut`), Lifetimes | 51 – 60 |
| **07** | [`07_errors_and_io/`](07_errors_and_io/) | No NULL, `Option<T>`, `Result<T, E>`, `?` operator, File I/O, modules | 61 – 70 |
| **08** | [`08_advanced_and_concurrency/`](08_advanced_and_concurrency/) | Generics, Traits, Closures, Iterators, `Box<T>`, Threads, Channels (`mpsc`) | 71 – 80 |

---

## 🎯 How to Learn
1. Read the lesson markdown file.
2. Open the matching practice task in `tasks/step_1_core_foundations/`.
3. Complete the task in `src/main.rs` and verify with `cargo run`.
4. If you are stuck, reveal one hint at a time. Compare with `solution/solution.rs` after trying. Run the answer with `cargo run --example solution` from that task folder.

An unfinished starter is not a completed exercise even when it compiles. Task 20 is the deliberate exception: its starter contains compiler errors for you to repair. Reading a lesson does not automatically mark it complete; explain the idea in your own words and finish its task first.
