use::std::net::TcpListener;
use::std::io::Read;
use crate::http::Request;
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
        
        loop {

            match listener.accept() {
                Ok((mut stream,_)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                             match Request::try_from(&buf[..]) {
                                 Ok(request) => {

                                 }
                                 Err(e) => {

                                 }
                             } // take u8 arr slice of full buf array to comply with &u8[] requirent
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