# Solution Explanation: Task 47 (Enums Holding Data)

Let's dissect how Rust enums bundle diverse data into a single type!

---

## 🔍 The Solution Code

```rust
#[derive(Debug)]
#[allow(dead_code)]
enum UserActivity {
    Login(String),
    Click { x: i32, y: i32 },
    Purchase { item: String, amount: f64 },
    Logout,
}

fn main() {
    println!("=== USER SESSION EVENT LOG ===");

    let a1 = UserActivity::Login(String::from("Zubair99"));
    let a2 = UserActivity::Click { x: 340, y: 720 };
    let a3 = UserActivity::Purchase {
        item: String::from("Wireless Headphones"),
        amount: 89.99,
    };
    let a4 = UserActivity::Logout;

    println!("Event 1: {:?}", a1);
    println!("Event 2: {:?}", a2);
    println!("Event 3: {:?}", a3);
    println!("Event 4: {:?}", a4);

    println!("==============================");
}
```

---

## 🧩 Key Takeaways

1. **Payload Diversity:**
   * `Login` holds a single `String`.
   * `Click` holds named `{ x, y }` coordinates.
   * `Purchase` holds a `String` and an `f64`.
   * `Logout` holds zero payload.
2. **Algebraic Data Types (Sum Types):**
   * Instead of creating complex class hierarchies with inheritance and pointers, Rust lets you bundle all possibilities cleanly inside one enum.

