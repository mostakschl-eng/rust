# Lesson 08: Variables — Storing Information in Memory Boxes

Welcome to Lesson 08! 

Up until now, we printed numbers and words directly on the screen.
Now we will learn how programs **remember information**: using **Variables**!

---

## 1. The Real-Life Analogy: The Labeled Storage Box

Imagine you are packing things in your house.
You have a cardboard box. You take a black Sharpie marker and write on the front:
> 📦 **`age`**

Then you drop the number `25` inside that box and close the lid.

Whenever someone asks: *"How old is Rahim?"*, you don't guess. You look for the box labeled **`age`**, open it, and read the number `25`!

In computer science:
* Your computer has memory (RAM) with billions of tiny storage spots.
* A **Variable** is simply a **labeled box** in your computer's memory holding a piece of information!

---

## 2. Creating a Variable with `let`

In Rust, you create a new variable box using the keyword **`let`**:

```rust
fn main() {
    let age = 25;
    let name = "Rahim";
    let country = "Bangladesh";

    println!("Name:    {}", name);
    println!("Age:     {}", age);
    println!("Country: {}", country);
}
```

### The Output on Screen:
```text
Name:    Rahim
Age:     25
Country: Bangladesh
```

---

## 3. Understanding Every Piece of the Code

Look at this line: `let age = 25;`

| Code Part | What does it mean? |
|---|---|
| `let` | Keyword telling Rust: *"Create a brand new variable box in memory."* |
| `age` | The **name** of the box. You choose the name. |
| `=` | The **assignment operator**. It takes the value on the right and drops it into the box on the left. |
| `25` | The value stored inside the box. |
| `;` | The semicolon marking the end of this instruction. |

---

## 4. Rust's Superpower: Type Inference

Did you notice that we didn't tell Rust that `25` is a number or `"Rahim"` is text?

Rust has **Type Inference** (smart detection).
When Rust sees `25`, it automatically knows: *"That's an integer (`i32`). I will make the box the right size for a number."*

If you want to be extra clear, you can also write the type explicitly:
```rust
let age: i32 = 25;
let name: &str = "Rahim";
```
Both ways are 100% correct in Rust!

---

## 5. Doing Math with Variables

Once data is stored in variable boxes, you can use those boxes in calculations:

```rust
fn main() {
    let apple_price = 3;
    let apple_count = 10;
    let total_cost = apple_price * apple_count;

    println!("Total cost for {} apples is ${}.", apple_count, total_cost);
}
```

### The Output on Screen:
```text
Total cost for 10 apples is $30.
```

---

## 6. Common Beginner Traps

### ❌ Trap 1: Using a Variable Before Creating It
```rust
fn main() {
    println!("{}", score); // Error! Rust does not know what 'score' is yet!
    let score = 100;
}
```
* **Why this fails:** Rust executes code from top to bottom. You must always create the box with `let` *before* you try to open and read it!

---

## 7. Summary Checklist
- [x] Variables are labeled storage boxes in computer memory.
- [x] Create a variable using: `let variable_name = value;`.
- [x] Rust automatically figures out what kind of data is inside the box.
- [x] You can do math with variables: `let total = a + b;`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/01_baby_steps/task_08_variables_boxes/TASK.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_08_variables_boxes/TASK.md)**.
1. Read the task requirements.
2. Create variables for a video game character profile (name, level, health, gold).
3. Run `cargo run` and display your character stats!
