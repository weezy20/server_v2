#![allow(unused)]
use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let mut package = String::with_capacity(4096);
    let mut server = TcpStream::connect("127.0.0.1:3000").unwrap();
    
    loop {
        println!("Enter input\n>");
        io::stdin().read_line(&mut package).expect("Failed to read from STDIN");
        // println!("Sending \"{}\" ...", package);

        if package.trim() == "quit".to_string() {
            println!("Exiting ...");
            std::process::exit(0);
        }
        // Send `package` as bytes into stream
        let bytes_sent = server.write(&mut package.as_bytes()).expect("Failed to write to server");
        package.clear();

        // Create a BufReader and read until `\n` char
        let mut reader = BufReader::new(&server);
        reader.read_line(&mut package).expect("Failed to read into buffer");

        println!("Server says : {}", package);
        package.clear();
        println!("{}", "*".repeat(20));
    }
}
