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

fn main() {
    println!("=== WEB SERVER ENDPOINT BUILDER ===");

    let api_server = Server::new(String::from("api.example.com"), 443);
    let local_server = Server::localhost(8080);

    println!("Production API Endpoint: {}", api_server.url());
    println!("Local Development Server: {}", local_server.url());

    println!("===================================");
}

