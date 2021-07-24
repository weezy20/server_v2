//! Provides HttpResponse and HttpRequest types. This crate
//! implements all logic regarding interconversion of raw byte
//!streans of HTTP responses and requests with the aforementioned types
//! 
//! An HTTP Request consists of HTTP method, HTTP version, and URI 

pub mod http_request;
pub mod http_response;