# Lesson 47: Enums Holding Data (`enum Message`)

Some languages use enums mainly as named integer constants. Rust enums can also attach a different payload to each variant.

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

    // 3. Variant with a char payload (like a tuple struct):
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
* **IPv6:** A 128-bit address, commonly written in hexadecimal text (e.g. `"::1"`)

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

Think of an enum as space for a payload plus information identifying its active variant. Its exact layout is **not** generally “largest payload plus one byte”: alignment, padding, tag size, and compiler optimizations affect the result. Sometimes unused bit patterns encode the variant without a separate tag. Do not rely on a fixed formula.

[Official type-layout reference](https://doc.rust-lang.org/reference/type-layout.html)

---

## 5. Quick Recap Checklist
- [x] Rust enum variants can store data directly inside them.
- [x] Variants can hold primitives, strings, tuples, or struct-like named fields.
- [x] Different variants of the same enum can hold completely different types.
- [x] Memory is safely managed using an internal discriminant tag.

---

## Pitfall: payloads need their own rules

`IpAddr::V6(String::from("not an address"))` still type-checks in this teaching model. A `String` does not validate an IPv6 address. Real programs can use `std::net::IpAddr` and parsing. Creating one variant requires exactly that variant’s fields, not fields from all variants.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `Login(String)` | Tuple-like variant carrying text |
| `Click { x, y }` | Variant carrying named fields |
| `Logout` | Variant without a payload |
| `{:?}` | Debug formatting |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_47_enums_with_data/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_47_enums_with_data/TASK.md).
* Next lesson: [Lesson 48: The `match` Expression: Exhaustive Matching](48_match_exhaustive.md).
