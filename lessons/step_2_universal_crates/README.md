# Step 2: The Universal Production Crates

Once you finish Step 1 (the 80 core lessons), you transition to using **crates** from the official registry ([crates.io](https://crates.io)). Almost every production company and codebase in the world uses these 3 fundamental packages:

---

## 📦 What You Will Learn in Step 2

### 1. `serde` & `serde_json` (Data Serialization & Deserialization)
* Parsing raw JSON strings into type-safe Rust structs with `#[derive(Serialize, Deserialize)]`.
* Exporting Rust data structures to JSON, YAML, and TOML.
* Handling missing or optional JSON fields.

### 2. `tokio` (Asynchronous Runtime)
* The difference between OS Threads (Lesson 79) and Async Tasks.
* `async fn` and the `.await` keyword.
* Running hundreds of thousands of concurrent I/O requests with minimal RAM.
* Tokio channels (`mpsc`, `broadcast`, `oneshot`).

### 3. `anyhow` & `thiserror` (Idiomatic Error Handling)
* When to use `anyhow::Result` for application code.
* When to use `thiserror` for writing custom reusable error enums in libraries.
* Contextual error chaining with `.context("Failed to load file")`.
