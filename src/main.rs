#![allow(dead_code)]
#![allow(unused_imports)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;

use std::env;

mod http;
mod server;
mod website_handler;


/**
 * env macro reads env variables provided by cargo at compile time.
 * CARGO_MANIFEST_DIR is the directory of Cargo.toml file.
 * We can see how code looks with vars inserted by running "cargo expand | code".
 */
fn main() {
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR")); 
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    print!("Serving files from: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

