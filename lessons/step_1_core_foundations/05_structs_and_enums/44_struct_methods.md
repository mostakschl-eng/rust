# Lesson 44: Methods: Attaching Behaviors to Structs (`impl`, `&self`)

Up until now, our structs were passive bags of data.  
If we wanted to calculate something with a struct, we had to write an independent standalone function:
This introductory fragment assumes the `Rectangle` definition shown in the complete example below.

```rust
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```
In modern software design, you want objects to know **how to perform their own actions**!  
* A `Rectangle` should be able to calculate its own `area()`.
* A `BankAccount` should be able to `deposit()` money into itself.

In Rust, we attach functions directly to structs using **Methods** and the **`impl` (implementation) block**! 📐

---

## 1. The Real-Life Analogy: The Smartphone 📱

Think of a modern smartphone:
* **Data (State):**
  * Battery Level: 85%
  * Brand: "RustPhone"
  * Storage: 256 GB
* **Abilities (Methods):**
  * `phone.make_call("01711...")`
  * `phone.charge(15)`
  * `phone.check_battery()`

You don't take your battery out and send it to an outside machine to check its charge.  
The phone **owns its data and provides methods to interact with it**!

---

## 2. Defining Methods with the `impl` Block

To define methods for a struct, use the `impl` (implementation) keyword followed by the struct name:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Attach methods to Rectangle:
impl Rectangle {
    // A method to calculate area:
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // A method to check if this rectangle can hold another inside it:
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // Call methods using dot syntax:
    println!("Rect1 Area: {} sq pixels", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### 📋 Expected Terminal Output:
```text
Rect1 Area: 1500 sq pixels
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Look how clean and readable `rect1.area()` is!

---

## 3. What is `self`? The Three Flavors of `self` 🍦

Notice the first parameter of every method: **`&self`**.  
The `self` parameter represents the specific instance on which the method was called!

Rust gives you three ways to take `self`:

| Self Variant | What It Does | When to Use It |
|---|---|---|
| `&self` | Borrows the instance immutably (read-only) | When you just need to inspect fields (e.g. `area()`) |
| `&mut self` | Borrows the instance mutably (can change fields) | When the method updates fields (e.g. `deposit()`, `resize()`) |
| `self` | Takes the receiver by value (moves a non-Copy instance; may transform or return it) | When transforming the instance into something else |

### Example of `&mut self` in Action:
```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Requires &mut self because balance is being modified!
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Rahim"),
        balance: 100.0,
    };

    account.deposit(50.0);
}
```
**Output:**
```text
Deposited $50.00. New balance: $150.00
```

---

## 4. Automatic Referencing and Dereferencing

In languages like C or C++, if you have a pointer to an object, you have to write `pointer->method()`, and if you have the object directly, you write `object.method()`.

Rust is much smarter!  
When you call `rect.area()`, Rust automatically inserts `&`, `&mut`, or `*` to match the signature of the method.  
For these ordinary method calls, write `instance.method()`. A mutable receiver still needs mutable access; automatic borrowing cannot bypass ownership rules.

---

## 5. Multiple `impl` Blocks

In Rust, you can have multiple `impl` blocks for the same struct across different parts of your code.  
This is super useful for organizing large codebases into modules:

```rust
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}

impl Rectangle {
    fn perimeter(&self) -> u32 { (self.width + self.height) * 2 }
}
```

---

## 6. Quick Recap Checklist
- [x] Methods are defined inside `impl StructName { ... }`.
- [x] A method has a `self` receiver; the common forms here are `&self`, `&mut self`, and `self`.
- [x] Call methods using dot notation: `instance.method_name()`.
- [x] Use `&self` to read fields, `&mut self` to modify fields.
- [x] Rust handles referencing/dereferencing automatically.

---

## Pitfalls and limits

`can_hold` here requires a strict fit (`>` on both dimensions), keeps orientation fixed, and does not allow rotation. `area()` uses `u32` arithmetic; these small inputs fit, but large dimensions may overflow. The account is a teaching model: real money needs exact units such as integer cents, validation, and access rules. A method does not stop code in the same module from editing fields directly.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `impl BankAccount` | Define associated behavior |
| `&self` | Borrow to inspect |
| `&mut self` | Borrow exclusively to modify |
| `-> bool` | Return success or failure |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_44_struct_methods/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_44_struct_methods/TASK.md).
* Next lesson: [Lesson 45: Associated Functions & Constructors (`Self::new`)](45_associated_functions.md).
