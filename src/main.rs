#![allow(dead_code)]

use server::Server;
use http::Request;
mod server;
mod http;

fn main() {
    let addr = "127.0.0.1:8081";

    // let port_index = addr.find(':').unwrap();
    // let port = &addr[port_index+1..];
    let server = Server::new(addr.to_string());
    server.run();
}




