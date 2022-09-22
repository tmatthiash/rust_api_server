use super::Request;
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};

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
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                println!("{:?}", request);
                            }
                            Err(e) => println!("Error: {}", e),
                        },
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
