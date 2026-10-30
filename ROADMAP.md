# Master Rust Learning Roadmap (`ROADMAP.md`)

This is your complete, step-by-step syllabus from complete beginner to writing real, reliable Rust programs. Check off `[x]` as you complete each topic!

---

## 🟢 Level 1: Absolute Foundations (In Progress)
*The core building blocks of computer programming.*

- [ ] **Lesson 01:** What is code? Your first Rust program (`fn main`, `{ }`, `println!`, `;`)
  - [ ] **Task 01:** Print custom greetings and multi-line output (`tasks/01_basics/task_01_hello/`)
- [ ] **Lesson 02:** What is a function? Reusable recipes (`fn`, parameters, return values)
  - [ ] **Task 02:** Build a personal greeter function
- [ ] **Lesson 03:** Variables — Storing information in boxes (`let`, immutable by default, `mut`)
  - [ ] **Task 03:** Counter and score keeper
- [ ] **Lesson 04:** Basic Data Types — What kind of data? (`i32`, `f64`, `bool`, `char`, `String`)
  - [ ] **Task 04:** Create a product price tag calculator
- [ ] **Milestone Project 1:** Personal Digital Business Card CLI

---

## 🟡 Level 2: Decision Making & Repeating (Logic)
*Teaching the computer how to think and repeat tasks.*

- [ ] **Lesson 05:** Conditions — Making decisions (`if`, `else if`, `else`)
  - [ ] **Task 05:** Age verification & ticket pricing
- [ ] **Lesson 06:** Comparisons & Boolean Logic (`==`, `!=`, `<`, `>`, `&&`, `||`)
  - [ ] **Task 06:** Login validator simulation
- [ ] **Lesson 07:** Loops — Doing repetitive work (`loop`, `while`, `break`)
  - [ ] **Task 07:** Countdown timer simulation
- [ ] **Lesson 08:** For Loops & Ranges (`for number in 1..=10`)
  - [ ] **Task 08:** Multiplication table generator
- [ ] **Lesson 09:** Pattern Matching with `match` (The cleaner `if/else`)
  - [ ] **Task 09:** Traffic light state machine
- [ ] **Milestone Project 2:** Secret Number Guessing Game

---

## 🟠 Level 3: Grouping Data (Structures)
*Organizing related pieces of data together.*

- [ ] **Lesson 10:** Tuples & Arrays (Fixed-size collections)
  - [ ] **Task 10:** Coordinates and 7-day temperature log
- [ ] **Lesson 11:** Vectors (`Vec<T>` — Resizable lists)
  - [ ] **Task 11:** Shopping cart list manager
- [ ] **Lesson 12:** Structs — Building custom data types (`struct User { ... }`)
  - [ ] **Task 12:** Bank account data model
- [ ] **Lesson 13:** Struct Methods (`impl` blocks — functions that belong to data)
  - [ ] **Task 13:** Bank account deposit & withdraw methods
- [ ] **Lesson 14:** Enums — Custom status choices (`enum OrderStatus { ... }`)
  - [ ] **Task 14:** E-commerce order tracker
- [ ] **Milestone Project 3:** Student Gradebook & GPA Calculator

---

## 🔵 Level 4: Rust's Superpower (Ownership & Borrowing)
*How Rust prevents crashes and manages memory safely without garbage collection.*

- [ ] **Lesson 15:** Memory basics: Stack vs Heap (Why memory matters)
- [ ] **Lesson 16:** Ownership Rules (Move semantics vs Copy types)
  - [ ] **Task 15:** Tracing ownership moves
- [ ] **Lesson 17:** References & Borrowing (`&` — looking without taking)
  - [ ] **Task 16:** Calculate string length by borrowing
- [ ] **Lesson 18:** Mutable References (`&mut` — changing borrowed data)
  - [ ] **Task 17:** Modify a list through a mutable reference
- [ ] **Lesson 19:** Slices (`&str` vs `String`)
  - [ ] **Task 18:** Extracting words from a sentence
- [ ] **Milestone Project 4:** Text Analyzer & Word Counter

---

## 🟣 Level 5: Real-World Rust (Error Handling & System I/O)
*Writing production-ready software that never crashes.*

- [ ] **Lesson 20:** The `Option<T>` Enum (Saying goodbye to `null` bugs)
  - [ ] **Task 19:** Finding items safely in a database
- [ ] **Lesson 21:** The `Result<T, E>` Enum (Handling errors like a pro)
  - [ ] **Task 20:** Safe math division without crashing
- [ ] **Lesson 22:** The Question Mark Operator (`?`) for clean error propagation
- [ ] **Lesson 23:** Reading User Input from the Keyboard (`std::io::stdin`)
  - [ ] **Task 21:** Interactive command-line prompt
- [ ] **Lesson 24:** Reading and Writing Files on Disk (`std::fs`)
  - [ ] **Task 22:** Save and load notes from a text file
- [ ] **Milestone Project 5:** Persistent Terminal Todo App (Saves tasks to disk)

---

## 🏆 Level 6: Capstone Project
*Building complete, standalone software.*

- [ ] **Project Choice A:** Weather / Currency CLI Tool (Calling an external API)
- [ ] **Project Choice B:** Mini HTTP Server / REST API from scratch
- [ ] **Project Choice C:** Markdown-to-HTML Static Site Generator
