# Solution Explanation: Task 45 (Associated Functions)

Let's dissect how constructors and factory functions work in Rust without needing a `new` keyword!

---

## 🔍 The Solution Code

```rust
struct Server {
    host: String,
    port: u16,
}

impl Server {
    fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    fn localhost(port: u16) -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port,
        }
    }

    fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
```

---

## 🧩 Key Takeaways

1. **Associated Functions have no `self`:**
   * `fn new(...) -> Self`
   * `Self` is an alias for the type `Server`.
2. **Double Colon `::` vs Dot `.`:
   * `Server::new(...)`: Associated function called on the type blueprint.
   * `server.url()`: Method called on an existing object instance.
3. **Alternative Named Constructors:**
   * `Server::localhost(port)` is a convenient factory method with sensible defaults prefilled.

