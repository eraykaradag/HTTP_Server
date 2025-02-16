use crate::http::Request;
use std::{fmt::Result, io::Read, net::TcpListener};
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Request Recieved: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("Failed to parse the request : {}", e)
                                }
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
