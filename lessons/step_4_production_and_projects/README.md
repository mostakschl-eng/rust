# Step 4: Real-World Portfolio Projects & Production Standards

In Step 4, you assemble everything you have learned to build real software applications to showcase on GitHub and master industrial best practices.

---

## 💼 The 3 Portfolio Projects

1. **Project 1: Production CLI Tool**
   * Built with `clap`, `reqwest`, `serde`, and local file caching.
   * Fetches data from a live public API (Crypto, Weather, or GitHub) and presents a formatted terminal report.

2. **Project 2: Full-Featured REST API with Database**
   * Built with `axum`, `sqlx`, and SQLite / PostgreSQL.
   * Features user authentication, CRUD operations, database migrations, and JSON validation.

3. **Project 3: Concurrent Real-Time Server**
   * Built with `tokio` async channels and WebSockets (`tungstenite`).
   * Handles multi-client broadcast messaging in real time.

---

## 🛡️ Industrial Quality Standards

* **`cargo clippy`**: Rust's automated linter that detects code smells and performance bottlenecks.
* **`cargo fmt`**: Auto-formats your entire workspace to Rust community standards.
* **Automated Testing (`#[test]`)**: Unit tests, integration tests in `tests/`, and benchmark profiling with `criterion`.
* **CI/CD Pipelines**: Setting up GitHub Actions (`.github/workflows/ci.yml`) to test every commit automatically.
