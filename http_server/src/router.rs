use std::net::TcpStream;

use http::{http_request::*, http_response::HttpResponse};

use super::handler::{PageNotFound, StaticHandler, WebServiceHandler};
/// Route a incoming request to the appropriate handler
/// Also call the `send_response` method on the stream to send back a response
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, mut stream: &TcpStream) {
        let route = req.get_url();
        let method = req.method;
        match method {
            Method::Get => {
                // A GET request maybe for a static page or a web service
                // Web services start with the keyword "api"
                let api_switch =
                    if route.split("/").into_iter().skip(1).next() == Some("api") {
                        true
                    } else {
                        false
                    };
                let response: HttpResponse = if api_switch {
                    WebServiceHandler::handle(&req)
                } else {
                    StaticHandler::handle(&req)
                };
                response.send_response(&mut stream);
            }
            // Method::Post => todo!(),
            // Method::Invalid => todo!(),
            _ => {
                let response: HttpResponse = PageNotFound::handle(&req);
                response.send_response(&mut stream);
            }
        }
    }
}
