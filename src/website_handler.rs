use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;
pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler  {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_name: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_name);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                     // ok() convert Result into String or None in not successful
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack: {}", file_name);
                    None
                }
            },
            Err(_) => None
        }
       
        
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
       

        match request.method() {

            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok,self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok,self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => {
                        Response::new(StatusCode::Ok,Some(content))
                    },
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),

        }
    }
}