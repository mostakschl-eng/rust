# Solution Explanation: Task 63 (Safe Option Unwrapping)

Let's dissect how safe unwrapping methods prevent unexpected crashes!

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
   * Unwraps `Some` safely. If it had been `None`, it would produce a detailed panic log with the exact error message specified.

