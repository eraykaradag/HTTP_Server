use crate::http::{request, ParseError, Request, Response, StatusCode};
use std::{fmt::Result, io::{Read, Write}, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse the request: {}",e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self, mut handler : impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Request Recieved: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
