//! Requests are incident on `Server` provided by this module

use super::router::Router;
use http::http_request::HttpRequest;
use std::io::{BufReader, Read};
use std::net::TcpListener;
/// Struct `Server` created with `Server::new(socket_addr)` will
/// start listening on `socket_addr` for incoming connections and convert
/// the incoming byte stream into `http::http_request::HttpRequest` for routing
pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection: TcpListener =
            TcpListener::bind(self.socket_addr).expect("Failed to bind to port");
        println!("Listening on {}", self.socket_addr);

        for stream in connection.incoming() {
            if let Ok(stream) = stream {
                println!(
                    "Connection established with {}",
                    stream.peer_addr().unwrap()
                );
                // we choose a buffer length of 256 bytes because this chapter says so:
                // https://www.w3.org/Protocols/rfc2616/rfc2616-sec3.html#sec3.2.1
                let mut buf = [0_u8; 256];
                match (&mut &stream).read(&mut buf) {
                    Ok(0) => continue,
                    Ok(bytes_read) => {
                        if !(bytes_read <= buf.len()) {
                            eprintln!("Request too long, aborting");
                            continue;
                        }
                        let request = String::from_utf8(buf.to_vec());
                        match request {
                            Ok(req) => {
                                Router::route(HttpRequest::from(&req), &stream);
                            }
                            Err(_) => {
                                eprintln!("Error parsing HTTP Request");
                                continue;
                            }
                        }
                    }
                    Err(_e) => {
                        eprintln!("Cannot read from socket");
                        continue;
                    }
                }
            }
        }
    }
}
