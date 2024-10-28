use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::error::AspirinEatsError;
use aspirin_eats::food::{Order, OrderRequest};
use aspirin_eats::http::{HttpRequest, HttpResponse};
use regex::Regex;
use std::{
    io::{Read, Write},
    net::TcpListener,
    str::{self, FromStr},
};

/// Change this path to match where you want to store the database file
const DB_PATH: &str =
    "/Users/anmolsandhu/github/courses/aspirin-2024-03/assignments/04-networking/aspirin_eats.db";

fn main() {
    let db = AspirinEatsDb::from_path(DB_PATH).expect("Failed to open database");
    let listener = TcpListener::bind("127.0.0.1:8080").expect("couldn't bind to address");
    let mut buf = [0; 1024];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Ok(amt) = stream.read(&mut buf) {
                    if amt == 0 {
                        // Connection was closed
                        continue;
                    }

                    // Attempt to parse the request
                    let request_str = match str::from_utf8(&buf[0..amt]) {
                        Ok(req) => req,
                        Err(_) => {
                            send_response(
                                &mut stream,
                                HttpResponse::from(AspirinEatsError::InvalidRequest),
                            );
                            continue;
                        }
                    };

                    let new_request = match HttpRequest::from_str(request_str) {
                        Ok(req) => req,
                        Err(_) => {
                            send_response(
                                &mut stream,
                                HttpResponse::from(AspirinEatsError::InvalidRequest),
                            );
                            continue;
                        }
                    };

                    println!("{:?}", new_request); // Retain for logging

                    // Route the request based on method
                    match new_request.method.as_deref() {
                        Some("GET") => handle_get(&db, &new_request, &mut stream),
                        Some("POST") => handle_post(&db, &new_request, &mut stream),
                        Some("DELETE") => handle_delete(&db, &new_request, &mut stream),
                        _ => send_response(
                            &mut stream,
                            HttpResponse::from(AspirinEatsError::MethodNotAllowed),
                        ),
                    }
                } else {
                    eprintln!("Failed to read from stream");
                }
            }
            Err(e) => {
                eprintln!("couldn't get client = {}", e);
            }
        }
    }
}

/// Sends an `HttpResponse` to the client.
fn send_response(stream: &mut std::net::TcpStream, response: HttpResponse) {
    let response_str = response.to_string();
    if let Err(e) = stream.write(response_str.as_bytes()) {
        eprintln!("couldn't write to stream: {}", e);
    }
}

/// Extracts the order ID from the path if it matches /orders/{id}
fn extract_order_id(path: &str) -> Option<i64> {
    let order_id_re = Regex::new(r"^/orders/(\d+)$").unwrap();

    if let Some(captures) = order_id_re.captures(path) {
        captures.get(1).map(|m| m.as_str().parse().unwrap())
    } else {
        None
    }
}

/// Handles GET requests.
fn handle_get(db: &AspirinEatsDb, request: &HttpRequest, stream: &mut std::net::TcpStream) {
    if let Some(ref path) = request.path {
        if path == "/orders" || path == "/orders/" {
            // Handle GET /orders
            match db.get_all_orders() {
                Ok(orders) => {
                    let body = serde_json::to_string(&orders).unwrap_or_else(|_| "[]".to_string());
                    send_response(&mut *stream, HttpResponse::new(200, "OK", &body));
                }
                Err(e) => {
                    send_response(
                        &mut *stream,
                        HttpResponse::from(AspirinEatsError::Database(e)),
                    );
                }
            }
        } else if let Some(id) = extract_order_id(path) {
            // Handle GET /orders/{id}
            match db.get_order(id) {
                Ok(Some(order)) => {
                    let body = serde_json::to_string(&order).unwrap();
                    send_response(&mut *stream, HttpResponse::new(200, "OK", &body));
                }
                Ok(None) => {
                    send_response(&mut *stream, HttpResponse::from(AspirinEatsError::NotFound));
                }
                Err(e) => {
                    send_response(
                        &mut *stream,
                        HttpResponse::from(AspirinEatsError::Database(e)),
                    );
                }
            }
        } else if path == "/" {
            // Handle GET /
            send_response(
                &mut *stream,
                HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!"),
            );
        } else {
            // Path not recognized
            send_response(&mut *stream, HttpResponse::from(AspirinEatsError::NotFound));
        }
    } else {
        send_response(
            &mut *stream,
            HttpResponse::from(AspirinEatsError::InvalidRequest),
        );
    }
}

/// Handles POST requests.
fn handle_post(db: &AspirinEatsDb, request: &HttpRequest, stream: &mut std::net::TcpStream) {
    if let Some(ref body) = request.body {
        // Deserialize the incoming JSON into OrderRequest
        let order_request_result: Result<OrderRequest, serde_json::Error> =
            serde_json::from_str(body);

        let order_request = match order_request_result {
            Ok(req) => req,
            Err(e) => {
                send_response(
                    &mut *stream,
                    HttpResponse::from(AspirinEatsError::ParseError(e)),
                );
                return;
            }
        };

        // Convert OrderRequest to Order
        let order: Order = order_request.into();

        // Insert the Order into the database
        match db.add_order(order.clone()) {
            Ok(id) => {
                let mut created_order = order;
                created_order.id = Some(id);
                let res_str = serde_json::to_string(&created_order).unwrap();
                send_response(&mut *stream, HttpResponse::new(201, "Created", &res_str));
            }
            Err(e) => {
                send_response(
                    &mut *stream,
                    HttpResponse::from(AspirinEatsError::Database(e)),
                );
            }
        }
    } else {
        send_response(
            &mut *stream,
            HttpResponse::from(AspirinEatsError::InvalidRequest),
        );
    }
}

/// Handles DELETE requests.
fn handle_delete(db: &AspirinEatsDb, request: &HttpRequest, stream: &mut std::net::TcpStream) {
    if let Some(ref path) = request.path {
        if path == "/orders" || path == "/orders/" {
            // Handle DELETE /orders
            match db.reset_orders() {
                Ok(_) => {
                    send_response(
                        &mut *stream,
                        HttpResponse::new(200, "OK", "All orders removed."),
                    );
                }
                Err(e) => {
                    send_response(
                        &mut *stream,
                        HttpResponse::from(AspirinEatsError::Database(e)),
                    );
                }
            }
        } else if let Some(id) = extract_order_id(path) {
            // Handle DELETE /orders/{id}
            match db.remove_order(id) {
                Ok(_) => {
                    send_response(&mut *stream, HttpResponse::new(200, "OK", "Order removed."));
                }
                Err(e) => {
                    send_response(
                        &mut *stream,
                        HttpResponse::from(AspirinEatsError::Database(e)),
                    );
                }
            }
        } else {
            // Path not recognized
            send_response(&mut *stream, HttpResponse::from(AspirinEatsError::NotFound));
        }
    } else {
        send_response(
            &mut *stream,
            HttpResponse::from(AspirinEatsError::InvalidRequest),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_order_id_valid() {
        let path = "/orders/42";
        let id = extract_order_id(path);
        assert_eq!(id, Some(42));
    }

    #[test]
    fn test_extract_order_id_invalid() {
        let path = "/orders/abc";
        let id = extract_order_id(path);
        assert_eq!(id, None);
    }

    #[test]
    fn test_extract_order_id_no_id() {
        let path = "/orders/";
        let id = extract_order_id(path);
        assert_eq!(id, None);
    }

    #[test]
    fn test_extract_order_id_wrong_path() {
        let path = "/order/42";
        let id = extract_order_id(path);
        assert_eq!(id, None);
    }
}
