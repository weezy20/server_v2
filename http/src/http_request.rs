#![allow(unused, non_snake_case)]
use std::collections::HashMap;
/// Crate http converts HTTP requests into struct HttpRequest

pub struct HttpRequest {
    /// The HTTP verb to be used
    pub method: Method,
    /// The HTTP Version to be used
    pub version: Version,
    /// The URL requested for
    pub resource: Resource,
    /// Additional headers identified by colon separated strings
    pub headers: HashMap<String, String>,
    /// Optional message body: POST requests may have a body whereas GET requests doesn't
    /// which is why body is an Option\<String\>
    pub body: Option<String>,
}
/// A unit struct that holds a URL
pub struct Resource(String);

/// Method::Get and Method::Post represent HTTP GET and POST methods
/// respectively. We derive PartialEq in order to allow comparison of
/// enum variants to generated variants in tests. Debug is derived for
/// future cases and is not necessary. From<&str> trait is derived to
/// directly generate the enum from a string.
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Invalid,
}
/// V1_1 represents HTTP version 1.1
/// V2_0 represents HTTP version 2.0
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Invalid,
}

/// Get a HTTP Method from string slice
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Invalid,
        }
    }
}
/// Get HTTP Version from string slice
impl From<&str> for Version {
    fn from(s : &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2" => Version::V2_0,
            _ => Version::Invalid,
        }
    }
}

/// The main logic of this module and the crate http. That is to parse
/// and create a HttpRequest object from bytes or string slice in this case
// Check From<&str> impl
impl From<String> for HttpRequest {
    // Read each line from string, delimited by CRLF \r\n, 
    // and identify if the line begins with "HTTP"
    // in which case it would be definitely an HTTP Request, then proceed to 
    // extract the version and method from the same line.
    //
    // If the line is a header line, then index the left subslice to the ":"
    // as a key and the right sublice as a value for the headers HashMap
    //
    // If line is empty \n\r, treat these as line separators
    // 
    // Optionally check for a message body, if present, store it.
    // \r\n\r\n marks the end of the request

    fn from(s: &str) -> Self {
        
    }
}
// Parses the HTTP request line and returns Method, Version and Resource
fn process_request_line(req: &str) -> (Method, Version, Resource) {

}
// Converts a "Key:Value" pair into a String tuple
fn process_header_line(header: &str) -> (String, String) {}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_FromTrait_impl() {
        let get: Method = "GET".into();
        assert_eq!(Method::Get, get);

        let post = Method::from("POST");
        assert_eq!(Method::Post, post);

        assert_eq!(Version::from("HTTP/2"), Version::V2_0);
    }
}
