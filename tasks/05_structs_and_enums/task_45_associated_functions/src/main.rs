struct Server {
    host: String,
    port: u16,
}

impl Server {
    // TODO 1: Associated function new(host: String, port: u16) -> Self
    fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    // TODO 2: Associated function localhost(port: u16) -> Self
    fn localhost(port: u16) -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port,
        }
    }

    // TODO 3: Method url(&self) -> String
    fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

fn main() {
    println!("=== WEB SERVER ENDPOINT BUILDER ===");

    // TODO 4: Call Server::new and Server::localhost
    let api_server = Server::new(String::from("api.example.com"), 443);
    let local_server = Server::localhost(8080);

    println!("Production API Endpoint: {}", api_server.url());
    println!("Local Development Server: {}", local_server.url());

    println!("===================================");
}

