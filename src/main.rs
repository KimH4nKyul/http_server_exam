use server::Server;
use http::request::Request;

fn main() {
    let addr = String::from("127.0.0.1:8080").to_string();
    let server = Server::new(addr);
    server.run();
}

mod server { 
    pub struct Server {
        addr: String,
    }
    
    impl Server {
        pub fn new(addr: String) -> Self {
            Self {
                addr
            }
        }
    
        pub fn run(self) {
            println!("Connection: {}", self.addr);
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>, 
            method: Method
        }
    }

    mod method { 
        pub enum Method {
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
    }
}