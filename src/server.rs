use::std::net::TcpListener;
use::std::io::{Read,Write};
use crate::http::{Request, Response, StatusCode};
use crate::http::request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
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
                                     dbg!(request);
                                     Response::new(
                                         StatusCode::Ok,
                                         Some("<h1>Hello</h1>".to_string())
                                        )
                                 }
                                 Err(e) => {
                                     Response::new(StatusCode::BadRequest, None)
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