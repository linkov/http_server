use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult, Display, Debug};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

/**
 * Use of lifetime 'buf
 * --------------------
 * In order to not allocate a copy of buffer parts
 * when we work with Request we need to use a reference to buffer.
 * In order for buffer to be available outside of normal ownership
 * pattern we must extend its lifetime
 * The solution is to use Rust compiler flag that associates lifetimes
 * of different objects.
 * 
 */

#[derive(Debug)]
pub struct Request<'buf> {
     path: &'buf str,
     query_string: Option<QueryString<'buf>>,
     method: Method
 }
 
 impl<'buf> Request<'buf> {
     // convention for getter is the same name as param
     pub fn path(&self) -> &str {
         &self.path
     }
     pub fn method(&self) -> &Method {
        &self.method
    }

    /**
     * Callers are interested in what is wrapped into Option
     * so instead of &Option<QueryString> we return ref to data behind option
     * so we return Option<&QueryString>
     *  */
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
 }

 impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
     type Error = ParseError;

     fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {

        // "?" means return error or wrapped value and try to convert utf8 error to ParseError, we do this with help of  impl From<Utf8Error> for ParseError
        let request = str::from_utf8(buf)?;
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // request var is set to this request in tuple
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // request var is set to this request in tuple
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // request var is set to this request in tuple

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // If we impl fromStr trait we get parse()
        let method: Method = method.parse()?;
        let mut query_string = None;
        
        // first solution
        // let question_mark_index = path.find('?');
        // if question_mark_index.is_some() {
        //     let index = question_mark_index.unwrap();
        //     query_string = Some(&path[index+1..]); // '?' is 1 byte
        //     path = &path[..index];
        // }

        // better solution
        if let Some(question_mark_index) = path.find('?') {
            query_string = Some( QueryString::from(&path[question_mark_index+1..])); // '?' is 1 byte
            path = &path[..question_mark_index];
        }

       Ok( Self {
           path,
           query_string,
           method
       })

     }
 }

 fn get_next_word(request: &str) -> Option<(&str, &str)> { // returns current word and rest of string
    
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..])) // space is 1 byte so i+1 is fine
        }
    }

    None
 }

 pub enum ParseError {
     InvalidRequest,
     InvalidEncoding,
     InvalidProtocol,
     InvalidMethod
 }

 impl ParseError {
     fn message(&self) -> &str {

        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
     }
 }

 impl From<MethodError> for ParseError {

    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

 impl From<Utf8Error> for ParseError {

    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

 impl Debug for ParseError {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}",self.message())
    }
 }

 impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}",self.message())
    }
 }

 impl Error for ParseError {
     
 }