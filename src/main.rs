fn main() {
    let addr = String::from("127.0.0.1:8080").to_string();
    let server = Server::new(addr);
    server.run();
}

mod server { 
    struct Server {
        addr: String,
    }
    
    impl Server {
        fn new(addr: String) -> Self {
            Self {
                addr
            }
        }
    
        fn run(self) {
            println!("Connection: {}", self.addr);
        }
    }
}

struct Request {
    path: String,
    query_string: Option<String>, 
    method: Method
}

enum Method {
    GET,
    POST,
    DELTE,
    PUT,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}