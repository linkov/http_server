#![allow(dead_code)]
#![allow(unused_imports)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    // we dont have to pass any params to Handler b/c its an empty struct
    server.run(WebsiteHandler);
}

