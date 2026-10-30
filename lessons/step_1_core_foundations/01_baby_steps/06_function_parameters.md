# Lesson 06: Function Parameters (Giving Ingredients to Your Functions)

Welcome to Lesson 06! 

In Lesson 04 and 05, our functions did the exact same thing every time (`make_coffee()` or `cheer()`).
Now, we are going to make functions **100 times more powerful** by giving them **inputs**!

---

## 1. The Real-Life Analogy: The Juice Blender

Imagine you have a kitchen blender.
* If you don't put anything inside the blender, it just spins empty blades.
* If you give it **Oranges**, it makes **Orange Juice**.
* If you give it **Mangoes**, it makes **Mango Juice**.

The blender machine is the **Function**.  
The named slots on the recipe are **parameters**; the actual fruits you supply are **arguments**.

> **A parameter** names an input in a function definition; an **argument** is a value supplied when calling it.

---

## 2. Your First Function with Input

Look at how we define an input in Rust:

```rust
fn greet_user(name: &str) {
    println!("Hello, {}! Welcome to the coding world.", name);
}

fn main() {
    greet_user("Rahim");
    greet_user("Sarah");
    greet_user("Alex");
}
```

### The Output on Screen:
```text
Hello, Rahim! Welcome to the coding world.
Hello, Sarah! Welcome to the coding world.
Hello, Alex! Welcome to the coding world.
```

Notice: One single function greeted three different people!

---

## 3. Breaking Down the Syntax: What is `name: &str`?

Look closely at the parentheses: `fn greet_user(name: &str)`

| Code Part | What does it mean? |
|---|---|
| `name` | The name of the input variable inside the function. |
| `:` (Colon) | Tells Rust: *"Here is the type of data this input must be."* |
| `&str` | Pronounced **"string slice"**. It is a borrowed view of UTF-8 text; string literals such as `"Rahim"` can be passed here. |

In Rust, you **must always tell the function what type of data to expect**. 
If a function expects text, you cannot give it a number by mistake. Rust guards against bugs before your code even runs!

---

## 4. Multiple Parameters (Giving Two or More Ingredients)

You can pass multiple parameters by separating them with commas `,`:

```rust
fn show_player_score(player_name: &str, score: i32) {
    println!("Player: {}\tScore: {} points", player_name, score);
}

fn main() {
    show_player_score("Rahim", 100);
    show_player_score("Sarah", 250);
}
```

### The Output on Screen:
```text
Player: Rahim	Score: 100 points
Player: Sarah	Score: 250 points
```

* **`player_name: &str`** accepts text.
* **`score: i32`** accepts a whole number (`i32` means 32-bit integer).

---

## 5. Common Beginner Traps

### ❌ Trap 1: Mismatched Data Type
```rust
show_player_score("Rahim", "one hundred"); // Error! Expected a number (i32), got text!
```
* **What Rust says:** `error: mismatched types: expected 'i32', found '&str'`.
* Rust checks this parameter type before running. A type-correct call can still contain a logic mistake, such as passing the wrong score.

---

## 6. Summary Checklist
- [x] Parameters are inputs placed inside the `()` parentheses of a function.
- [x] Each parameter needs a name and a type: `name: Type`.
- [x] `&str` is used for text inputs.
- [x] `i32` is used for whole number inputs.
- [x] Separate multiple parameters with commas `,`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/01_baby_steps/task_06_function_inputs/TASK.md`](../../../tasks/step_1_core_foundations/01_baby_steps/task_06_function_inputs/TASK.md)**.
1. Read the task requirements.
2. Build a product receipt printer function that takes item name, quantity, and price.
3. Run `cargo run` and test with different products!
