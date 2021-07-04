#![allow(unused)]
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let mut package = String::with_capacity(4096);
    let mut buf = [0u8; 4096];
    loop {
        let mut server = TcpStream::connect("127.0.0.1:3000").unwrap();
        println!("Enter input");
        io::stdin().read_line(&mut package).unwrap();
        println!("Status of package : {} ", package);

        if package.trim() == "quit".to_string() {
            println!("Exiting ...");
            std::process::exit(0);
        }
        let bytes_sent = server.write(&mut package.as_bytes()).unwrap();
        package.clear();

        let bytes_recvd = server.read(&mut buf).unwrap();

        assert_eq!(bytes_recvd, bytes_sent);

        println!("Server says : {}", String::from_utf8_lossy(&buf));
        println!("{}", "*".repeat(20));
    }
}
