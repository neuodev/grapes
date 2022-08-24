use std::{collections::HashMap, net::TcpStream};

pub struct Response {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: StatusCode::Ok,
            headers: HashMap::new(),
            body: None,
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

    pub fn body<T: Into<String>>(&mut self, body: T) -> &mut Self {
        self.body = Some(body.into());

        self
    }

    pub fn send(&mut self, socket: TcpStream) {}
}

/// For the full list check https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
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
