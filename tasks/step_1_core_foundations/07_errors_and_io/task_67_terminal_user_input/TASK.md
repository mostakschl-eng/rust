# Task 67: Terminal User Greeter (Standard Input std::io::stdin)

## 🎯 What is Your Task?
Write a Rust program to clean, sanitize, and parse raw terminal text input:
1. Define a helper function `parse_terminal_input(raw: &str) -> (String, u32)`:
   - Trims surrounding whitespace and newlines with `.trim()`.
   - Splits on whitespace into parts.
   - Returns the first token as the name (or `Guest`) and the second token as a u32 age (or `18` if missing/invalid). These defaults are demonstration policy, not validation of a real person.
2. In `main()`:
   - Print a prompt, create a mutable String buffer, and actually call `io::stdin().read_line(&mut buffer)`. Return or report I/O errors.
   - Print clean name and age.
   - Use `Guest` when a name is missing. Treat EOF like an empty line for this demo. Ignore tokens after age. Add silent parser assertions for ordinary input, empty input, invalid, negative, and out-of-range ages.

---

## 📋 Expected Output on Your Screen

Type `  Tanvir 22` and press Enter when prompted. The following shows program output; an interactive terminal also echoes your input.
```text
=== TERMINAL INPUT SANITIZER ===
Enter a name and age (for example: Tanvir 22):
Sanitized Name:  Tanvir
Parsed Age:      22 years old
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Clean raw text using `.trim()`.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_67_terminal_user_input
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Create a mutable String, print the prompt, then actually read into it and handle the I/O Result.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Split whitespace, use Guest for a missing name, parse the second token as u32, and use 18 only for this explicitly requested demo fallback.

</details>

**Before running:** read_line returns a byte count; zero means EOF. trim removes all surrounding whitespace, not just Enter. This exercise permits one-word names and demo defaults, not identity or age verification.
