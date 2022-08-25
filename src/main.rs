#[allow(unused, dead_code)]
mod request;
mod response;

use std::env;
use std::io::{BufReader, Read};
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;

use crate::request::Request;
use response::{Response, StatusCode};

const BUF_SIZE: usize = 512;
pub const STATIC_DIR: &str = "./static";

fn main() -> Result<(), std::io::Error> {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "127.0.0.1:4242".to_string());
    println!("[addr] {}", addr);
    let server = Server::new(&addr);
    server.run()?;
    Ok(())
}

struct Server {
    addr: SocketAddr,
}

impl Server {
    fn new(addr: &str) -> Self {
        let addr = SocketAddr::from_str(&addr).expect("Invalid socket address");
        Self { addr }
    }

    fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.addr)?;

        while let Ok((mut stream, addr)) = listener.accept() {
            println!("[conn] {}", addr);
            let mut buf_reader = BufReader::new(&stream);

            let mut req_buf: Vec<u8> = vec![];
            let mut buf;
            loop {
                buf = [0; BUF_SIZE];
                match buf_reader.read(&mut buf) {
                    Ok(n) => {
                        req_buf.extend(&buf[..n]);
                        if n < BUF_SIZE {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            // Todo: Send 400 for invalid request
            let req = Request::new(req_buf).unwrap();

            let mut res = Response::new();
            res.status_code(StatusCode::Ok)
                .header("Content-Type", "text/html")
                .serve_file()
                .send(&mut stream)?;
        }

        Ok(())
    }
}
