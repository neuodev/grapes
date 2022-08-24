use std::env;
use std::io::{BufRead, BufReader, Write};
#[allow(unused, dead_code)]
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;

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

        while let Ok((mut socket, addr)) = listener.accept() {
            println!("[conn] {}", addr);
            let buf_reader = BufReader::new(&socket);
            let http_request = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|l| !l.is_empty())
                .collect::<Vec<_>>();
            println!("[request] {:#?}", http_request);

            let res = "HTTP/1.1 200 OK\r\n\r\n";
            socket.write_all(res.as_bytes())?;
        }

        Ok(())
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
