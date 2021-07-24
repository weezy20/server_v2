#![allow(unused)]
use core::str;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let socket = TcpListener::bind("127.0.0.1:3000").unwrap();
<<<<<<< HEAD
    // let mut buf = [0u8; 4096];
    let mut buf_string = String::new();
=======
>>>>>>> 96c9a7f7ae2b1feffecd05b2729253ad0673fdc7
    for stream in socket.incoming() {
        if let Ok(mut stream) = stream {
            println!(
                "connection established with {}",
                stream.peer_addr().unwrap()
            );
<<<<<<< HEAD
            let mut reader = BufReader::new(&stream);
            loop {
                // loop is required to keep working with the stream it is connected to

                let bytes_read = reader.read_line(&mut buf_string).unwrap();
                if bytes_read == 0 {
                    continue;
                }
                let bytes_written = (&stream).write(buf_string.as_bytes()).unwrap();
                println!("{} bytes written", bytes_written);
                buf_string.clear();
            }
        } // stream dropped here
=======
            let mut buf = [0u8; 4096];
            let bytes_read = stream.read(&mut buf).unwrap();
            stream.write(&mut buf[..bytes_read]).unwrap();
        }
>>>>>>> 96c9a7f7ae2b1feffecd05b2729253ad0673fdc7
    }
}
