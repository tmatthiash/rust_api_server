#![allow(dead_code)]

use http::Request;
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let addr = "127.0.0.1:8081";

    let default_path = format! {"{}/public", env!("CARGO_MANIFEST_DIR")}; //compile time env
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); //runtime env
    println!("the public path is {}", public_path);
    let server = Server::new(addr.to_string());
    server.run(WebsiteHandler::new(public_path));
}
