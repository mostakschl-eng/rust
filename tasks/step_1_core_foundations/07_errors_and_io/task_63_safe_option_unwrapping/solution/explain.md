# Solution Explanation: Task 63 (Safe Option Unwrapping)

Let's dissect how safe unwrapping methods make missing-value policies explicit!

---

## 🔍 The Solution Code

```rust
fn main() {
    let custom_theme: Option<&str> = None;
    let active_theme = custom_theme.unwrap_or("Cyberpunk Dark");

    let cached_port: Option<u16> = None;
    let service_port = cached_port.unwrap_or_else(|| 8080);

    let root_cert: Option<&str> = Some("CERT_SHA256_ROOT_OK");
    let cert = root_cert.expect("CRITICAL: Root certificate missing from bundle!");
}
```

---

## 🧩 Key Takeaways

1. **`.unwrap_or()`:**
   * Returns `"Cyberpunk Dark"` because `custom_theme` was `None`. Zero panics.
2. **`.unwrap_or_else()`:**
   * The fallback closure `|| 8080` is only evaluated if `cached_port` is `None`. Ideal for expensive computations.
3. **`.expect()`:**
   * Extracts the value from the fixed `Some` in this example. If it had been `None`, it would produce a detailed panic log with the exact error message specified.


## Reading the Result Carefully

A missing paint color can use the shop default. A missing identity document cannot be replaced with a made-up identity.

| Symbol or term | Meaning |
|---|---|
| `.unwrap_or(v)` | eager fallback |
| `.unwrap_or_else(f)` | lazy fallback |
| `.expect(msg)` | panic if absent |

expect is a presence assertion, not certificate verification. This exercise uses a fixed Some value; real external certificate errors need deliberate error handling.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
