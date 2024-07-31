#![allow(dead_code)]

use server::Server;
mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8090".to_string());
    server.run();
}


