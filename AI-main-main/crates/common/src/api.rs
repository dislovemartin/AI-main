
use std::collections::HashMap;

/// A generic HTTP request structure.
pub struct HttpRequest {
    pub endpoint: String,
    pub params: HashMap<String, String>,
}

/// A generic HTTP response structure.
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Handles a basic request and returns a default response.
pub fn handle_request(req: HttpRequest) -> HttpResponse {
    match req.endpoint.as_str() {
        "/ping" => HttpResponse {
            status: 200,
            body: "Pong!".to_string(),
        },
        _ => HttpResponse {
            status: 404,
            body: "Not Found".to_string(),
        },
    }
}
