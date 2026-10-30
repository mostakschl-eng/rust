# Task 45: Web Server Endpoint Builder (Associated Functions)

## 🎯 What is Your Task?
Write a Rust program with constructor-style associated functions to instantiate servers:
1. Define a struct `Server`:
   - `host: String`
   - `port: u16`
2. In `impl Server`:
   - Associated function `new(host: String, port: u16) -> Self`: constructs a custom server.
   - Associated function `localhost(port: u16) -> Self`: constructs a server with host `"127.0.0.1"`.
   - Method `url(&self) -> String`: returns `format!("http://{}:{}", self.host, self.port)`.
3. In `main()`:
   - Create `api_server = Server::new(String::from("api.example.com"), 443);`
   - Create `local_server = Server::localhost(8080);`
   - Print the URLs for both servers using their `.url()` method.

---

## 📋 Expected Output on Your Screen
```text
=== WEB SERVER ENDPOINT BUILDER ===
Production API Endpoint: http://api.example.com:443
Local Development Server: http://127.0.0.1:8080
===================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_45_associated_functions/src/main.rs).
2. Write associated functions without `self`, returning `Self`.
3. Write `url(&self)` method taking `&self`.
4. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_45_associated_functions
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_45_associated_functions/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_45_associated_functions/solution/explain.md)

