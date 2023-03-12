#![allow(dead_code)]

use server::Server;

mod server;
mod http;

fn main() {
    let addr = String::from("127.0.0.1:8080").to_string();
    let server = Server::new(addr);
    server.run();
}