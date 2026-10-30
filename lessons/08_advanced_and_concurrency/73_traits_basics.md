# Lesson 73: Traits: Defining Shared Behavior (`trait`)

In object-oriented programming (OOP) languages like Java or C#, shared behavior is often implemented using class inheritance (`class Dog extends Animal`).  
However, deep inheritance trees frequently lead to brittle, tangled architectures (known as the "fragile base class problem").

Rust does not have class inheritance!  
Instead, Rust uses **Traits**! 📜✨

---

## 1. The Real-Life Analogy: The Driver's License 🪪

Think about having a Driver's License:
* A **Human** is not a Car or a Truck.
* Having a Driver's License means you have mastered a **specific capability**:
  * You know how to `steer()`.
  * You know how to `brake()`.
  * You know how to `accelerate()`.
* It doesn't matter if you are 20 years old, 50 years old, tall, or short.
* As long as you implement the **Driver capability**, the state will allow you to drive on the highway!

In Rust:
> A **Trait** tells the Rust compiler about functionality a particular type has and can share with other types.  
> It defines an **abstract contract** of method signatures that types must fulfill!

---

## 2. Defining a Trait

Use the `trait` keyword followed by the trait name and method signatures:

```rust
// Define the contract: any type that implements 'Summarizable' must provide a 'summarize' method!
pub trait Summarizable {
    fn summarize(&self) -> String;
}
```

Notice there is no method body `{}`—just a semicolon `;`.  
It is an unfilled promise waiting for concrete types to implement it!

---

## 3. Implementing a Trait for a Struct

To fulfill the contract, use the syntax:  
**`impl TraitName for StructName`**:

```rust
pub trait Summarizable {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        format!("'{}' by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2024 Edition Released!"),
        author: String::from("Ferris"),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Learning Rust is fun and rewarding! 🦀"),
    };

    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary:   {}", tweet.summarize());
}
```

### 📋 Expected Terminal Output:
```text
Article Summary: 'Rust 2024 Edition Released!' by Ferris
Tweet Summary:   @rustlang: Learning Rust is fun and rewarding! 🦀
```

Two completely different structs (`NewsArticle` and `Tweet`) now share the exact same capability: `.summarize()`!

---

## 4. Default Method Implementations

Traits don't just have to be empty contracts. You can provide a **default method body**!  
Types implementing the trait can choose to use the default or override it:

```rust
pub trait Describable {
    // Default implementation:
    fn describe(&self) -> String {
        String::from("(Default item description)")
    }
}

struct Sensor;
impl Describable for Sensor {} // Uses default!

fn main() {
    let s = Sensor;
    println!("{}", s.describe()); // (Default item description)
}
```

---

## 5. Traits as Function Parameters: `impl Trait`

You can write functions that accept **any type that implements a specific trait**:

```rust
// This function will accept a NewsArticle, a Tweet, or ANY type that implements Summarizable!
fn print_summary(item: &impl Summarizable) {
    println!("Breaking News Summary: {}", item.summarize());
}
```

---

## 6. Quick Recap Checklist
- [x] Traits define shared interfaces and capabilities.
- [x] Declared with `trait TraitName { fn method(&self) -> ReturnType; }`.
- [x] Implemented on types using `impl TraitName for TypeName`.
- [x] Traits can provide optional default method implementations.
- [x] Use `&impl Trait` to write functions that accept any type fulfilling the trait.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_73_traits_basics/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_73_traits_basics/TASK.md).
* Next lesson: [Lesson 74: Implementing Traits for Custom Types](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/74_implementing_traits.md).
