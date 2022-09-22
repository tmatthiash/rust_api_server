use crate::http::Response;
use crate::http::StatusCode;

use super::Request;
use std::convert::TryFrom;
use std::{io::Read, io::Write, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("listening at {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => Response::new(
                                    StatusCode::Ok,
                                    Some("<h1>It works</h1>".to_string()),
                                ),
                                Err(e) => {
                                    println!("Error: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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
