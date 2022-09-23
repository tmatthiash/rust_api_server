use crate::http::ParseError;
use crate::http::Response;
use crate::http::StatusCode;

use super::Request;
use std::convert::TryFrom;
use std::{io::Read, io::Write, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        unimplemented!()
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request {}", err);
        Response::new(StatusCode::BadRequest, None)        
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("listening at {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e)
                        }
                    }
                }
                Err(e) => println!("Err {}", e.to_string()),
            }
        }
    }
}
