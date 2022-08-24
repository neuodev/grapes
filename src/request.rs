use std::{
    collections::HashMap,
    string::{self, FromUtf8Error},
};

use regex::Regex;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    query: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn new(buf: Vec<u8>) -> Result<Request, &'static str> {
        let request = match String::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => return Err("Invalid UTF-8"),
        };
        println!("{:#?}", request);
        let re_info = Regex::new("(?P<method>.+) (?P<path>.*) (?P<version>.+)").unwrap();
        let re_headers = Regex::new("(?P<key>[^:\n]+): (?P<value>.+)").unwrap();

        let caps = match re_info.captures(&request) {
            Some(c) => c,
            None => return Err("Invalid request"),
        };

        let method = Method::new(&caps["method"])?;
        let path = caps["path"].to_string();
        let version = &caps["version"];

        if version.to_uppercase() != "HTTP/1.1" {
            return Err("Only HTTP/1.1 is supported");
        }

        let mut headers = HashMap::new();
        re_headers.captures_iter(&request).for_each(|cap| {
            let key = cap["key"].trim().to_string();
            let value = cap["value"].trim().to_string();
            headers.insert(key, value);
        });

        // Extract URI and query strings
        let re_uri = Regex::new(r"^(?P<uri>[^\?\n]+)").unwrap();
        let re_query = Regex::new(r"(?P<key>[^=&\n\?]+)=(?P<value>[^=&\n]+)").unwrap();
        let mut query = HashMap::new();
        re_query.captures_iter(&request).for_each(|caps| {
            let key = caps["key"].trim().to_string();
            let value = caps["value"].trim().to_string();
            query.insert(key, value);
        });

        let body = request.split("\r\n\r\n").skip(1).collect::<Vec<_>>();
        let body = match body.get(0) {
            Some(b) if b.is_empty() => None,
            Some(b) => Some(b.to_string()),
            None => return Err("Invalid request body"),
        };

        Ok(Request {
            body,
            headers,
            method,
            path,
            query,
        })
    }
}

#[derive(Debug)]
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

impl Method {
    fn new(m: &str) -> Result<Self, &'static str> {
        let method = match m {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            _ => return Err("Unknown method"),
        };

        Ok(method)
    }
}
