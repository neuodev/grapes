pub struct Response {
    
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
    HttpVersionNotSupported = 505
}

