# Lesson 41: Custom Types: Named Structs (`struct`)

Welcome to **Level 5: Structs, Enums & Pattern Matching**! 🏛️

In Level 4, we learned about **Tuples**:
```rust
let user = ("Rahim", "rahim@example.com", 28, true);
```
While tuples bundle data together, they have a big problem:
* What is `user.0`?
* What is `user.2`? Is it an age? An ID? A score?
* If someone swaps fields of the **same type**, the compiler cannot infer their intended meanings.

To write clean, enterprise-grade software, we need **self-documenting custom data types**.  
In Rust, we create these using **Structs**!

---

## 1. The Real-Life Analogy: The Official Identity Card / Passport 🪪

Think of a National ID Card or Student ID:
* It doesn't just print numbers randomly on plastic.
* It has **labeled fields**:
  * **Full Name:** Rahim Ahmed
  * **Email:** rahim@example.com
  * **Age:** 28
  * **Is Active:** true

In Rust:
> A **Struct** (short for "structure") is a custom blueprint that allows you to name and package related pieces of data together into one unified type.

---

## 2. Defining and Creating Your First Struct

### Step A: Define the Blueprint
Use the `struct` keyword with curly braces `{ }`, defining each field name and its type:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}
```

### Step B: Create an Instance
To create a concrete user, state the struct name and supply values for every field:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("rahim99"),
        email: String::from("rahim@example.com"),
        sign_in_count: 1,
        is_active: true,
    };

    // Access fields using clean dot notation:
    println!("Username:      {}", user1.username);
    println!("Email Address: {}", user1.email);
    println!("Logins:        {}", user1.sign_in_count);
    println!("Active:        {}", user1.is_active);
}
```

### 📋 Expected Terminal Output:
```text
Username:      rahim99
Email Address: rahim@example.com
Logins:        1
Active:        true
```

Look how much more readable `user1.email` is compared to `user.1`!

---

## 3. The Field Init Shorthand: Clean & Elegant!

If your function parameters have the **exact same name** as the struct fields, you don't need to repeat yourself (`username: username`):

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // Shorthand for email: email
        username,   // Shorthand for username: username
        is_active: true,
        sign_in_count: 1,
    }
}
```

---

## 4. Mutability: The Entire Struct is Mutable

In Rust, mutability is applied to the **entire instance**, not to individual fields:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("alice_dev"),
        email: String::from("alice@oldmail.com"),
        sign_in_count: 5,
        is_active: true,
    };

    // We can update the email because user1 is declared with 'mut':
    user1.email = String::from("alice@newmail.com");
    user1.sign_in_count += 1;

    println!("Updated email: {}", user1.email);
    println!("Total logins:  {}", user1.sign_in_count);
}
```

> [!NOTE]
> Rust does not allow marking only one single field as `mut`. If `user1` is mutable, all its fields are mutable.

---

## 5. Printing Entire Structs with `#[derive(Debug)]`

If you try to print `println!("{:?}", user1);`, the compiler will complain:
`User does not implement Debug`.

To fix this, simply add **`#[derive(Debug)]`** directly above your struct definition:

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let user = User {
        username: String::from("codemaster"),
        email: String::from("code@rust.org"),
        sign_in_count: 42,
        is_active: true,
    };

    // Standard Debug:
    println!("Debug: {:?}", user);

    // Pretty-print with line breaks and indentations:
    println!("Pretty:\n{:#?}", user);
}
```

### 📋 Expected Terminal Output:
```text
Debug: User { username: "codemaster", email: "code@rust.org", sign_in_count: 42, is_active: true }
Pretty:
User {
    username: "codemaster",
    email: "code@rust.org",
    sign_in_count: 42,
    is_active: true,
}
```

---

## 6. Summary of Struct Syntax

| Syntax | Purpose | Example |
|---|---|---|
| `struct Name { ... }` | Defines the struct blueprint | `struct Book { title: String }` |
| `Name { field: val }` | Instantiates a struct | `Book { title: String::from("Rust") }` |
| `instance.field` | Reads a specific field | `user.email` |
| `#[derive(Debug)]` | Enables `{:?}` printing | Put directly above `struct Name` |
| `field,` | Shorthand initialization | `email,` instead of `email: email,` |

---

## 7. Common Beginner Traps ⚠️

### Trap: Missing a Field During Creation
When instantiating a struct, you **must supply every single field**. Fields are not implicitly left uninitialized. Later you can deliberately obtain defaults with `Default` and update syntax:
```rust
let u = User {
    username: String::from("bob"),
    // ❌ ERROR: missing `email`, `sign_in_count`, `is_active`
};
```

---

## 8. Quick Recap Checklist
- [x] Structs bundle named fields of different types together.
- [x] Access fields using `instance.field_name`.
- [x] Use field init shorthand when variable names match field names.
- [x] Mutability applies to the entire struct instance (`let mut instance = ...`).
- [x] Add `#[derive(Debug)]` above the struct to enable `{:?}` and `{:#?}` printing.

---

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `struct Student { ... }` | Define named fields |
| `Student { ... }` | Construct a value |
| `student1.gpa` | Access one field |
| `#[derive(Debug)]` | Generate debug formatting |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_41_named_structs/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_41_named_structs/TASK.md).
* Next lesson: [Lesson 42: Creating & Updating Struct Instances (`..` syntax)](42_struct_instances.md).
