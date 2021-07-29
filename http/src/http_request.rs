//! Convert incoming HTTP requests into struct HttpRequest
#![allow(unused, non_snake_case)]
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
/// Type representing HTTP Request
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
#[derive(Debug, PartialEq)]
pub struct Resource(String);

/// `Method::Get` and `Method::Post` represent HTTP GET and POST methods
/// respectively. We derive `PartialEq` in order to allow comparison of
/// enum variants to generated variants in tests. Debug is derived for
/// future cases and is not necessary. `From<&str>` trait is derived to
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
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2" => Version::V2_0,
            _ => Version::Invalid,
        }
    }
}

impl From<&str> for Resource {
    fn from(s: &str) -> Self {
        Resource(s.to_string())
    }
}

/// The main logic of this module and the crate http. That is to parse
/// and create a HttpRequest object from bytes or string slice in this case
// Check From<&str> impl
impl From<&String> for HttpRequest {
    /// Read each line from string, delimited by CRLF \r\n,
    /// and identify if the line begins with "HTTP" (Request Line)
    /// in which case it would be definitely an HTTP Request, then proceed to
    /// extract the version and method from the same line.
    ///
    /// If the line is a header line, then index the left subslice to the ":"
    /// as a `key` and the right sublice as a `value` for the headers hashmap.
    ///
    /// If line is empty \n\r, treat these as line separators
    ///
    /// Optionally check for a message body, if present, store it.
    /// \r\n\r\n marks the end of the request

    fn from(s: &String) -> Self {
        // the "p" stands for "parsed", so read p_method as "parsed_method" and so on..
        let mut p_method = Method::Invalid;
        let mut p_version = Version::Invalid;
        let mut p_resource = Resource("".to_string());
        let mut p_headers: HashMap<String, String> = HashMap::new();
        let mut p_body: Option<String> = None;

        // Read each line of the incoming stream
        for line in s.lines() {
            // Process for HTTP request line
            if line.contains("HTTP") {
                let (_method, _version, _resource) = process_request_line(&line);
                p_method = _method;
                p_version = _version;
                p_resource = _resource;
                continue;
            }

            // Process for header line
            if line.contains(":") {
                let (key, value) = process_header_line(&line);
                p_headers.insert(key, value);
                continue;
            }

            // Ignore empty lines.
            if line.len() != 0 {
                // Maybe there is a body
                if let Some(mut previous_lines) = p_body {
                    previous_lines.push_str(line);
                    p_body = Some(previous_lines);
                } else {
                    p_body = Some(String::from(line));
                }
            }
        }
        HttpRequest {
            method: p_method,
            version: p_version,
            resource: p_resource,
            headers: p_headers,
            body: p_body,
        }
    }
}
/// Parses the HTTP request line and returns Method, Version and Resource
fn process_request_line(req: &str) -> (Method, Version, Resource) {
    // Safety: We can unwrap here safely because this message is only called
    // on valid HTTP requests
    let mut req = req.split_whitespace().into_iter();
    let method = req.next().unwrap();
    let resource = req.next().unwrap();
    let version = req.next().unwrap();

    let method = Method::from(method);

    let resource = Resource::from(resource);

    let version = Version::from(version);

    (method, version, resource)
}

/// Converts a "Key:Value" String into tuple
fn process_header_line(header: &str) -> (String, String) {
    let split_index = header.find(':').unwrap();
    let k = String::from(&header[..split_index]);
    let v = String::from(&header[split_index + 1..]);
    (k, v)
}

#[cfg(test)]
mod tests {
    use crate::http_request;

    use super::*;
    #[test]
    fn test_FromTrait_impl() {
        let get: Method = "GET".into();
        assert_eq!(Method::Get, get);

        let post = Method::from("POST");
        assert_eq!(Method::Post, post);

        assert_eq!(Version::from("HTTP/2"), Version::V2_0);
    }

    #[test]
    fn test_httprequest() {
        let s = format!(
            "{} {} {}Host: localhost:3000\r\n{}Accept: */*\r\n\r\n",
            "GET", "/index.html", "HTTP/1.1\r\n", "User-Agent: curl/1.1.1\r\n",
        );

        let test = HttpRequest::from(&s);

        // expected values
        let _method = Method::from("GET");
        let _version = Version::from("HTTP/1.1");
        let _resource = Resource::from("/index.html");
        let mut _headers: HashMap<String, String> = HashMap::new();
        // mind the leading whitespace in the following values
        _headers.insert("User-Agent".into(), " curl/1.1.1".into());
        _headers.insert("Accept".into(), " */*".into());
        _headers.insert("Host".into()," localhost:3000".into());
        // expected values 

        assert_eq!(test.method, _method);
        assert_eq!(test.version, _version);
        assert_eq!(test.resource, _resource);
        assert_eq!(test.headers, _headers);

    }
}
