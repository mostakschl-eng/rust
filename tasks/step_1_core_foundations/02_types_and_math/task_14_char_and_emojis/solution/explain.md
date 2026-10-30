# Task 14: Solution Breakdown & Unicode Representation

## 1. Line-by-Line Breakdown
* In Rust, `'🦀'` is a 4-byte Unicode scalar value.
* Ferris the Crab is the unofficial friendly mascot of the Rust programming language!
* Single quotes `' '` ensure the Rust compiler treats this as one `char`. Double-quoted string literals are borrowed UTF-8 text (`&str`), not automatically heap allocations. Some visible emojis require multiple scalar values.

