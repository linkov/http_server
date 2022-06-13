use super::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IOResult};
use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // concrete impl takes TcpStream
    // pub fn send(&self, stream: &mut TcpStream) -> IOResult<()> {
    //     let b = match &self.body {
    //         Some(b) => b,
    //         None => ""
    //     };
    //     // instead of Formatter that created string on heap we write directly to stream
    //     write!(
    //         stream,
    //         "HTTP/1.1 {} {}\r\n\r\n{}",
    //         self.status_code,
    //         self.status_code.reason_phrase(),
    //         b
    //     )
    // }

    // more generic impl takes a trait (which is like an interface or a protocol in Swift)
    // "dyn" stands for "dynamic dispatch" - compiler decides what concrete impl to call
    // during dynamic dispatch compiled code calls lookup vtable instead of calling concrete impl directly which means runtime overhead
    // pub fn send(&self, stream: &mut dyn Write) -> IOResult<()> {
    //     let b = match &self.body {
    //         Some(b) => b,
    //         None => ""
    //     };
    //     // instead of Formatter that created string on heap we write directly to stream
    //     write!(
    //         stream,
    //         "HTTP/1.1 {} {}\r\n\r\n{}",
    //         self.status_code,
    //         self.status_code.reason_phrase(),
    //         b
    //     )
    // }

    /**
     * To deal with overhead Rust provides "static displatch", the lookup is done at compile time
     * concrete type inserted in each fn call based on param type
     * Downside is slower compile time + larger binary which can be a problem on embedded systems
     *  */ 

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let b = match &self.body {
            Some(b) => b,
            None => ""
        };
        // instead of Formatter that created string on heap we write directly to stream
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            b
        )
    }


}
