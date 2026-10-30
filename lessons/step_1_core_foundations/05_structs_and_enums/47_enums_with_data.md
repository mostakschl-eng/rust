# Lesson 47: Enums Holding Data (`enum Message`)

In most programming languages (C, C++, C#, Java), an `enum` is just a glorified list of numbers: `Red = 0`, `Yellow = 1`, `Green = 2`.  
They cannot hold extra data.

In Rust, Enums are a superpower known in computer science as **Algebraic Data Types**:
> **In Rust, each individual enum variant can hold its own unique types and quantities of data!** 🚀

---

## 1. The Real-Life Analogy: The Postal Delivery Service 📬

Imagine a courier service handling customer deliveries:
1. **A Postcard:** Contains recipient address and a short text note.
2. **A Heavy Parcel:** Contains recipient address, weight in kilograms, and insurance value in dollars.
3. **A Legal Document Envelope:** Contains a tracking code and priority flag.
4. **An Empty Return Slip:** Contains zero data!

All 4 are **Deliveries**, but each one carries **completely different cargo**!

In Rust:
We don't need 4 separate classes and inheritance hierarchies. We define **one single Enum** with data attached to each variant!

---

## 2. Defining Enums with Data

Look at how diverse variants can be:

```rust
#[derive(Debug)]
enum WebEvent {
    // 1. Variant with no data (like a unit struct):
    PageLoad,

    // 2. Variant with named fields (like a struct):
    Click { x: i64, y: i64 },

    // 3. Variant with a String payload (like a tuple struct):
    KeyPress(char),

    // 4. Variant with multiple numbers:
    Resize(u32, u32),
}

fn main() {
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 120, y: 350 };
    let key = WebEvent::KeyPress('Q');
    let resize = WebEvent::Resize(1920, 1080);

    println!("{:?}", load);
    println!("{:?}", click);
    println!("{:?}", key);
    println!("{:?}", resize);
}
```

### 📋 Expected Terminal Output:
```text
PageLoad
Click { x: 120, y: 350 }
KeyPress('Q')
Resize(1920, 1080)
```

---

## 3. Real-World Case Study: IP Addresses 🌐

Every computer on the internet connects via an IP address.  
There are two major standards:
* **IPv4:** Four 8-bit numbers (e.g. `127.0.0.1`)
* **IPv6:** A long hexadecimal string (e.g. `"::1"`)

Look at how cleanly Rust expresses this reality:

```rust
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IPv4:     {:?}", home);
    println!("Loopback IPv6: {:?}", loopback);
}
```

> [!TIP]
> Notice how `IpAddr::V4` takes four `u8` bytes, while `IpAddr::V6` takes a `String`. Rust enums let you store completely different data structures under one clean type roof!

---

## 4. How Much Memory Does an Enum Use? 🧠

You might wonder: *"If one variant is a tiny boolean and another is a large 64-byte payload, how does Rust allocate memory?"*

Rust uses a **Tagged Union**:
* Size of the enum = **Size of the largest variant + a tiny 1-byte discriminant tag**.
* The tag tells Rust which variant is currently active.
* This gives you total type safety with optimal memory efficiency!

---

## 5. Quick Recap Checklist
- [x] Rust enum variants can store data directly inside them.
- [x] Variants can hold primitives, strings, tuples, or struct-like named fields.
- [x] Different variants of the same enum can hold completely different types.
- [x] Memory is safely managed using an internal discriminant tag.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/05_structs_and_enums/task_47_enums_with_data/TASK.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_47_enums_with_data/TASK.md).
* Next lesson: [Lesson 48: The `match` Expression: Exhaustive Matching](file:///d:/rust-learning/lessons/05_structs_and_enums/48_match_exhaustive.md).
