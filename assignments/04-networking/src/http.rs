use crate::error::AspirinEatsError;
use regex::Regex;
use std::{fmt::Display, str::FromStr};

/// Simple wrapper for an HTTP Request
#[derive(Debug)]
pub struct HttpRequest {
    /// The HTTP method used in the request (GET, POST, etc)
    pub method: Option<String>,

    /// The path requested by the client
    pub path: Option<String>,

    /// The body of the request
    pub body: Option<String>,
}

impl FromStr for HttpRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Regular expression to capture the HTTP method, path, and HTTP version
        let re = Regex::new(r"^(GET|POST|PUT|DELETE|PATCH) ([^ ]+) HTTP/1\.[01]")
            .map_err(|e| e.to_string())?;

        // Separate headers and body
        let parts: Vec<&str> = s.split("\r\n\r\n").collect();
        let (header, body) = match parts.len() {
            2 => (parts[0], Some(parts[1].to_string())),
            1 => (parts[0], None),
            _ => return Err("Invalid HTTP request format".to_string()),
        };

        // Capture the method and path from the header
        let caps = re
            .captures(header)
            .ok_or("Failed to parse HTTP method and path")?;
        let method = caps.get(1).map(|m| m.as_str().to_string());
        let path = caps.get(2).map(|p| p.as_str().to_string());

        Ok(HttpRequest { method, path, body })
    }
}

pub struct HttpResponse {
    status_code: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, body: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            body: body.to_string(),
        }
    }
}

impl Display for HttpResponse {
    /// Convert an HttpResponse struct to a valid HTTP Response
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code, self.status_text, self.body
        )
    }
}

impl From<AspirinEatsError> for HttpResponse {
    /// Given an error type, convert it to an appropriate HTTP Response
    fn from(value: AspirinEatsError) -> Self {
        let (status_code, status_text, body) = match value {
            AspirinEatsError::ParseError(_) => (400, "Bad Request", "Invalid Request"),
            AspirinEatsError::Database(_) => {
                (500, "Internal Server Error", "Internal Server Error")
            }
            AspirinEatsError::Io(_) => (500, "Internal Server Error", "Internal Server Error"),
            AspirinEatsError::InvalidRequest => (400, "Bad Request", "Invalid Request"),
            AspirinEatsError::NotFound => (404, "Not Found", "Resource not found"),
            AspirinEatsError::MethodNotAllowed => (405, "Method Not Allowed", "Method not allowed"),
        };

        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            body: body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_from_str() {
        let request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\nthis is the body.";
        let http_request = HttpRequest::from_str(request).unwrap();
        assert_eq!(http_request.method, Some("GET".to_string()));
        assert_eq!(http_request.path, Some("/orders".to_string()));
        assert_eq!(http_request.body, Some("this is the body.".to_string()));
    }

    #[test]
    fn test_http_response_to_string() {
        let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!");
        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\n\r\nWelcome to Aspirin Eats!"
        );
    }

    #[test]
    fn test_http_response_from_aspirin_eats_error() {
        let error = AspirinEatsError::InvalidRequest;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 400);
        assert_eq!(response.status_text, "Bad Request");
        assert_eq!(response.body, "Invalid Request");

        let error = AspirinEatsError::NotFound;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 404);
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(response.body, "Resource not found");

        let error = AspirinEatsError::MethodNotAllowed;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 405);
        assert_eq!(response.status_text, "Method Not Allowed");
        assert_eq!(response.body, "Method not allowed");

        let error = AspirinEatsError::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 500);
        assert_eq!(response.status_text, "Internal Server Error");
        assert_eq!(response.body, "Internal Server Error");
    }
}
