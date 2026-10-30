# Lesson 42: Creating & Updating Struct Instances (`..` syntax)

In Lesson 41, we created struct instances from scratch by typing out every single field manually.  
What if you want to create a new instance that shares **90% of its fields** with an existing instance, changing only one or two values?

Rust gives us the **Struct Update Syntax (`..`)**! 🔄

---

## 1. The Real-Life Analogy: Photocopying a Form with One Change 📄

Imagine you work in an office:
* You have a pre-filled 10-page employee contract for **Alice**:
  * Department: Engineering
  * Office Location: Floor 4, Desk 12
  * Company: Rust Corp
  * Health Plan: Premium Gold
* Now **Bob** joins the exact same engineering team!
* You don't want to type out all 10 pages again from scratch.
* You photocopy Alice's document, but write **Bob's name and email** on the first line!

In Rust:
> The **Struct Update Syntax (`..existing_instance`)** copies or moves all the remaining unspecified fields from an existing struct into your new instance!

---

## 2. The Struct Update Syntax in Action

Look at how concise this makes your code:

```rust
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
    enable_tls: bool,
    environment: String,
}

fn main() {
    // 1. Base production server configuration:
    let prod_config = ServerConfig {
        host: String::from("192.168.1.10"),
        port: 443,
        max_connections: 10_000,
        enable_tls: true,
        environment: String::from("Production"),
    };

    // 2. Create staging server: Change ONLY host and environment; copy the rest!
    let staging_config = ServerConfig {
        host: String::from("10.0.0.5"),
        environment: String::from("Staging"),
        ..prod_config // Copy port, max_connections, and enable_tls from prod_config!
    };

    println!("=== STAGING CONFIGURATION ===");
    println!("Host:            {}", staging_config.host);
    println!("Port:            {}", staging_config.port);
    println!("Max Connections: {}", staging_config.max_connections);
    println!("TLS Enabled:     {}", staging_config.enable_tls);
    println!("Environment:     {}", staging_config.environment);
}
```

### 📋 Expected Terminal Output:
```text
=== STAGING CONFIGURATION ===
Host:            10.0.0.5
Port:            443
Max Connections: 10000
TLS Enabled:     true
Environment:     Staging
```

---

## 3. The Golden Placement Rule: `..` Must Be Last!

The `..existing_instance` syntax must always appear at the **very bottom** of the struct creation block:

```rust
// ✅ Correct: Specify your custom fields first, then copy the rest
let new_cfg = ServerConfig {
    port: 8080,
    ..old_cfg
};

// ❌ COMPILER ERROR: ..old_cfg cannot come before custom fields!
let bad_cfg = ServerConfig {
    ..old_cfg,
    port: 8080,
};
```

---

## 4. Crucial Memory Insight: Ownership and Moved Fields 🧠

This is an important detail:
* If the fields being copied implement the **`Copy`** trait (like numbers `u16`, `u32`, and booleans `bool`), the data is duplicated safely.
* But if a non-`Copy` field such as **`String`** is supplied via `..old_instance`, ownership of that string is **moved** into the new struct!

Look at what happens:
```rust
struct Profile {
    username: String,
    score: i32,
}

fn main() {
    let p1 = Profile {
        username: String::from("Alice"),
        score: 100,
    };

    let p2 = Profile {
        score: 200,
        ..p1 // ⚠️ p1.username is MOVED into p2!
    };

    println!("P2 username: {}", p2.username); // Works!
    // println!("P1 username: {}", p1.username); // ❌ ERROR: borrow of moved value: `p1.username`
}
```

> [!TIP]
> If you provided your own new `username` in `p2`, then `p1.username` wouldn't be moved, and `p1` would remain fully usable!

---

## 5. Factory Helper Functions

It is common practice in Rust to write small helper functions that return configured structs:

```rust
fn create_default_server(host: String) -> ServerConfig {
    ServerConfig {
        host,
        port: 8080,
        max_connections: 500,
        enable_tls: false,
        environment: String::from("Development"),
    }
}
```

---

## 6. Quick Recap Checklist
- [x] Use `..instance` at the bottom of a struct literal to copy remaining fields.
- [x] The `..` syntax must always come last.
- [x] `Copy` fields (such as numbers and booleans) are copied; non-`Copy` fields (such as `String`) are moved unless explicitly overridden. Storage location alone does not decide this.
- [x] Struct update syntax eliminates repetitive boilerplate!

---

## Pitfall: a partial move

After `..p1` moves `p1.username`, that field and the complete `p1` value cannot be used. An unmoved field such as `p1.score` remains usable here. In the server example both String fields are explicitly replaced, so only `Copy` fields are taken and the original remains usable. The helper above is a factory function; a builder pattern usually has a separate builder value and stepwise configuration.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `..phone_base` | Supply unspecified fields from the base |
| `Copy` | Trait allowing implicit copies |
| `String` | Owned text that moves unless explicitly replaced |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/05_structs_and_enums/task_42_struct_instances/TASK.md`](../../../tasks/step_1_core_foundations/05_structs_and_enums/task_42_struct_instances/TASK.md).
* Next lesson: [Lesson 43: Tuple Structs & Unit-Like Structs](43_tuple_and_unit_structs.md).
