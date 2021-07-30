//! Main web server that incorporates a main() function, socket server, handler
//! and router, and manages the coordinations among them. It may serve html as well as JSON.
//! Here's how this works : `Server` receives a request, it passes it to a Router,
//! The router then selects and appropriate handler to process the request and send a Response,
//! meanwhile our crate `http` provides all the necessary interconversions.
//! 
//! This crate will bind to a socket and listen for incoming connections

mod router;
mod server;
mod handler;
use server::Server;
fn main(){
    let addr = "localhost:3000";
    let server = Server::new(addr);    
    server.run();
}