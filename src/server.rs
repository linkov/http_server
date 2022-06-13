use::std::net::TcpListener;
use::std::io::{Read,Write};
use request::ParseError;

use crate::http::{Request, Response, StatusCode};
use crate::http::request;
use std::convert::TryFrom;
use std::convert::TryInto;

// A trait is like an interface or a protocol in Swift
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // We can provide default impls, for example for very generic cases
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self, mut hadler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on: {}", &self.addr);
        loop {

            match listener.accept() {
                Ok((mut stream,_)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            let response = match Request::try_from(&buf[..]) {
                                 Ok(request) => {
                                     hadler.handle_request(&request)
                                 }
                                 Err(e) => {
                                     hadler.handle_bad_request(&e)
                                     
                                 }
                             };


                             if let Err(e) = response.send(&mut stream) {
                                 println!("Failed to send response: {}", e);
                             }
                        }
                        Err(e) => {
                            println!("Failed to read buf: {}", e);

                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                    continue;
                }
            }
            
        }
    }
}