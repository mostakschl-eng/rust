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
The fruits you drop inside the blender are called **Parameters** (or **Arguments**).

> **A Parameter** is an input you pass into a function so it can work with different data!

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
| `&str` | Pronounced **"string slice"**. In Rust, this means: **Text**. |

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
* **Why this is awesome:** In other languages (like Python or JavaScript), passing the wrong type might crash the program while running in front of a real customer. In Rust, the compiler catches it instantly before you run it!

---

## 6. Summary Checklist
- [x] Parameters are inputs placed inside the `()` parentheses of a function.
- [x] Each parameter needs a name and a type: `name: Type`.
- [x] `&str` is used for text inputs.
- [x] `i32` is used for whole number inputs.
- [x] Separate multiple parameters with commas `,`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/01_baby_steps/task_06_function_inputs/TASK.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_06_function_inputs/TASK.md)**.
1. Read the task requirements.
2. Build a product receipt printer function that takes item name, quantity, and price.
3. Run `cargo run` and test with different products!
