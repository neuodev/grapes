use std::{collections::HashMap, io::Write, net::TcpStream, fs, path::Path};

use crate::STATIC_DIR;

pub struct Response {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert("Connection".into(), "keep-alive".into());
        default_headers.insert("Keep-Alive".into(), "timeout=5".into());
        Self {
            status_code: StatusCode::Ok,
            headers: default_headers,
            body: vec![],
        }
    }

    pub fn status_code(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = status;

        self
    }

    pub fn headers<K: Into<String>, V: Into<String>>(&mut self, headers: Vec<(K, V)>) -> &mut Self {
        headers.into_iter().for_each(|(key, value)| {
            self.header(key, value);
        });

        self
    }

    pub fn header<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) -> &mut Self {
        self.headers.insert(key.into(), value.into());

        self
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.as_bytes().to_vec();

        self
    }

    pub fn send(&mut self, socket: &mut TcpStream) -> Result<(), std::io::Error> {
        let mut res = String::from(format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code as u16,
            self.status_code.as_str()
        ));


        self.headers
            .iter()
            .for_each(|(key, value)| res.push_str(format!("{}: {}", key, value).as_str()));

        res.push_str("\r\n\r\n");

        let mut res = res.as_bytes().to_vec();
        res.extend(&self.body);

        socket.write_all(&res[..])?;

        Ok(())
    }

    pub fn serve_file(&mut self, uri: &str) -> &mut Self {

        let mut file = uri;

        if file == "/" {
            file = "/index.html"
        }

        let path = format!("{}/{}", STATIC_DIR, file);
        let path = Path::new(&path);

        match fs::read(path) {
            Ok(f) => {
                self.body = f;
            },
            Err(_) => {
                self.body = format!("<strong>{} not found</strong>", uri).as_bytes().to_vec();
                self.status_code(StatusCode::NotFound);

            }
        }
        self
    }
}

/// For the full list check https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    // Successful responses
    Ok = 200,
    Created = 201,
    Accepted = 202,
    // Client error responses
    BadRequest = 400,
    UnAuthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    // Server error responses
    ServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    Unavailable = 503,
    HttpVersionNotSupported = 505,
}

impl StatusCode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            _ => todo!(),
        }
    }
}