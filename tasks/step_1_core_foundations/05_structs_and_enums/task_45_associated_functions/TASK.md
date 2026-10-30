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

1. Open [`src/main.rs`](src/main.rs).
2. Write associated functions without `self`, returning `Self`.
3. Write `url(&self)` method taking `&self`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_45_associated_functions
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Constructors take ordinary parameters and return a Server via `Self`.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

`localhost` supplies the host itself; `url(&self)` uses `format!` to return text without changing the server.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Printing a URL does not start a server. The http scheme stays http even on port 443; a port number alone does not enable TLS. These fixed hosts are demonstration values, not a general URL builder.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
