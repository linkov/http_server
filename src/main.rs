use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;


fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

