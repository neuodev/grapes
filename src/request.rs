use std::collections::HashMap;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
    headers: HashMap<String, String>,
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
