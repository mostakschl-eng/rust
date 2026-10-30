# Lesson 46: Enums: Representing Fixed Choices (`enum`)

Up until now, we've used **Structs** to bundle multiple pieces of data that exist together (like a user having a name AND email AND age).

What if a concept can only be **ONE OF SEVERAL FIXED POSSIBILITIES**?
* A traffic light is either **Red, Yellow, OR Green**.
* A player's direction is either **North, South, East, OR West**.
* An order status is either **Pending, Shipped, Delivered, OR Cancelled**.

In Rust, this is called an **Enumeration** (or **Enum** for short)! 🚦

---

## 1. The Real-Life Analogy: The Traffic Light 🚦

Look at an intersection traffic light:
* It has 3 glass lenses: Red, Yellow, and Green.
* At any given second, the signal is **strictly ONE of those three**:
  * It can never be Red AND Green at the same time.
  * It can never be Purple or Pink.
  * There are no secret undefined states.

In Rust:
> An **Enum** defines a custom type by listing all of its possible **variants**.  
> An instance of an enum can only ever be **one single variant at a time**!

---

## 2. Defining and Using an Enum

Use the `enum` keyword with curly braces `{ }`, separating variants with commas:

```rust
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    // 1. Create enum instances using the :: namespace operator:
    let current_signal = TrafficLight::Red;
    let next_signal = TrafficLight::Green;

    println!("Current signal: {:?}", current_signal);
    println!("Next signal:    {:?}", next_signal);

    // 2. Comparison:
    if current_signal == TrafficLight::Red {
        println!("STOP! Do not cross! 🛑");
    }
}
```

### 📋 Expected Terminal Output:
```text
Current signal: Red
Next signal:    Green
STOP! Do not cross! 🛑
```

---

## 3. Why Enums Beat Plain Strings or Numbers 🛡️

Why not just use strings like `let signal = "red";` or numbers `let signal = 1;`?

### The String Disaster:
```rust
let signal = "redd"; // Typo!
let signal = "GREEN"; // Uppercase mismatch!
let signal = "banana"; // Completely nonsensical state!
```
The computer won't complain until your self-driving car crashes into an intersection!

### The Rust Enum Guarantee:
```rust
let signal = TrafficLight::Purple; 
// ❌ COMPILER ERROR: no variant named `Purple` found in enum `TrafficLight`
```
With enums, invalid states are **physically impossible to compile**! You get 100% compile-time safety.

---

## 4. Attaching Methods to Enums with `impl`!

Just like structs, you can attach methods directly to an enum using an `impl` block:

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn action_advice(&self) -> &str {
        match self {
            TrafficLight::Red => "Stop and wait.",
            TrafficLight::Yellow => "Prepare to stop safely.",
            TrafficLight::Green => "Proceed with caution.",
        }
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    println!("The light is {:?}: {}", light, light.action_advice());
}
```

### 📋 Expected Terminal Output:
```text
The light is Yellow: Prepare to stop safely.
```

---

## 5. Quick Recap Checklist
- [x] Enums define a type with a fixed set of named variants.
- [x] Access variants using `EnumName::Variant`.
- [x] An enum instance is only ever ONE variant at any time.
- [x] Prevents typos, illegal states, and runtime bugs at compile time.
- [x] Enums can have `impl` blocks with methods!

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/05_structs_and_enums/task_46_enums_basics/TASK.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_46_enums_basics/TASK.md).
* Next lesson: [Lesson 47: Enums Holding Data (`enum Message`)](file:///d:/rust-learning/lessons/05_structs_and_enums/47_enums_with_data.md).
