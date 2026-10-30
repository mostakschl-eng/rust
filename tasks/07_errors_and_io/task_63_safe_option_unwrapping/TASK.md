# Task 63: App Configuration Defaults (Safe Option Unwrapping)

## 🎯 What is Your Task?
Write a Rust program to load application configurations with safe fallbacks and explicit unwraps:
1. Load UI Theme:
   ```rust
   let custom_theme: Option<&str> = None;
   ```
   - Use `.unwrap_or("Cyberpunk Dark")` to provide a default theme safely.
2. Load Network Port:
   ```rust
   let cached_port: Option<u16> = None;
   ```
   - Use `.unwrap_or_else(|| 8080)` to lazily compute fallback port `8080`.
3. Load Security Certificate:
   ```rust
   let root_cert: Option<&str> = Some("CERT_SHA256_ROOT_OK");
   ```
   - Use `.expect("CRITICAL: Root certificate missing from bundle!")` to unwrap the certificate.
4. Print all three resolved values.

---

## 📋 Expected Output on Your Screen
```text
=== SYSTEM CONFIGURATION LOADER ===
Active Theme:      Cyberpunk Dark (Defaulted safely)
Service Port:      8080 (Computed lazily)
Security Cert:     CERT_SHA256_ROOT_OK (Verified via expect)
===================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_63_safe_option_unwrapping/src/main.rs).
2. Use `.unwrap_or()`, `.unwrap_or_else()`, and `.expect()`.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_63_safe_option_unwrapping
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_63_safe_option_unwrapping/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_63_safe_option_unwrapping/solution/explain.md)

