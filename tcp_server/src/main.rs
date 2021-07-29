#![allow(unused)]
use core::str;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let socket = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in socket.incoming() {
        if let Ok(mut stream) = stream {
            println!(
                "connection established with {}",
                stream.peer_addr().unwrap()
            );
            let mut buf = [0u8; 4096];
            let bytes_read = stream.read(&mut buf).unwrap();
            stream.write(&mut buf[..bytes_read]).unwrap();
        }
    }
}
