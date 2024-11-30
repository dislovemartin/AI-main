use std::collections::HashMap;

/// Represents a simple HTTP request.
pub struct HttpRequest {
    pub endpoint: String,
    pub params: HashMap<String, String>,
}

/// Represents a simple HTTP response.
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Handles an incoming HTTP request and returns a response.
pub fn handle_request(req: HttpRequest) -> HttpResponse {
    match req.endpoint.as_str() {
        "/status" => HttpResponse { status: 200, body: "Service is running".to_string() },
        _ => HttpResponse { status: 404, body: "Endpoint not found".to_string() },
    }
}
