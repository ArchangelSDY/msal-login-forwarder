use core::fmt;
use std::net::TcpListener;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use warp::{Filter, http::Response};

#[derive(Serialize, Deserialize)]
struct ForwardRequest {
    url: String,
    port: u16,
}

impl fmt::Display for ForwardRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForwardRequest<URL: {}, Port: {}>",
            self.url, self.port)
    }
}

#[tokio::main]
async fn main() {
    let forward = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::json())
        .map(|forward_request: ForwardRequest| {
            println!("{}", forward_request);

            // Launch browser
            if let Err(e) = webbrowser::open(&forward_request.url) {
                return Response::builder()
                    .status(500)
                    .body(e.to_string());
            }

            // Start temp server
            let addr = format!("0.0.0.0:{}", forward_request.port);
            let server = match TcpListener::bind(addr) {
                Ok(s) => s,
                Err(e) => return Response::builder()
                    .status(500)
                    .body(e.to_string()),
            };

            let mut stream = match server.accept() {
                Ok((stream, _addr)) => stream,
                Err(e) => return Response::builder()
                    .status(500)
                    .body(e.to_string()),
            };

            let mut reader = std::io::BufReader::new(&mut stream);
            let mut line = String::new();
            if let Err(e) = reader.read_line(&mut line) {
                return Response::builder()
                    .status(500)
                    .body(e.to_string());
            };
            let parts: Vec<&str> = line.split_whitespace().collect();
            let path = parts[1];
            let query = &path[2..path.len()];
            println!("Query: {}", query);

            if let Err(e) = stream.write(b"HTTP/1.1 200 OK\r\n\r\nSuccess!") {
                return Response::builder()
                    .status(500)
                    .body(e.to_string());
            }
            if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
                return Response::builder()
                    .status(500)
                    .body(e.to_string());
            }

            // Send back
            return Response::builder()
                .body(query.to_string());
        });

    println!("Listening...");

    warp::serve(forward)
        .run(([0, 0, 0, 0], 9080))
        .await
}
