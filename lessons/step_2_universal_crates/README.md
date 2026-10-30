# Step 2: Selected libraries

This is a planned outline. Full lessons and task packages have not been written yet. The existing folder name is kept so links and the learning structure stay stable.

1. Learn [Cargo dependencies](https://doc.rust-lang.org/cargo/guide/dependencies.html): read Cargo.toml, choose a version, understand optional features, and read a library’s documentation.
2. Work with JSON when the project needs it. Explore [serde and serde_json](https://serde.rs/) for converting between supported Rust values and JSON. Learn how to handle invalid or missing data.
3. Continue Result and clear error messages. Consider [anyhow](https://docs.rs/anyhow/latest/anyhow/) for application errors or [thiserror](https://docs.rs/thiserror/latest/thiserror/) for custom error types if they help your project. Neither is required for every program.
4. Learn async/await only when the project needs asynchronous I/O. Consider [Tokio](https://tokio.rs/tokio/tutorial) after understanding that need. It is optional, and performance depends on the workload and resources.

Choose libraries for a problem rather than learning a universal required stack. Start with the standard library when it is enough. Continue the checking habits begun in Step 1.

[Back to the four-step roadmap](../../ROADMAP.md)
