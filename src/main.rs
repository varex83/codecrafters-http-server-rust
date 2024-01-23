mod request;
mod response;

use crate::request::HttpRequest;
use crate::response::{HttpResponse, StatusCode};
use std::fmt::Display;
use tokio::net::TcpListener;

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = vec![
            format!("HTTP/1.1 {} OK\r\n", self.status),
            "\r\n".to_string(),
        ]
        .concat();

        write!(f, "{}", lines)
    }
}

async fn process_socket(socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("Connection from {}", addr);

    // read the request

    let mut buf = [0; 1024];
    let n = socket.try_read(&mut buf).unwrap();
    let request = String::from_utf8(buf[0..n].to_vec()).unwrap();

    println!("Request: {}", request);

    let parsed_request = HttpRequest::try_from(request).unwrap();

    println!("Parsed request: {:?}", parsed_request);

    // write the response

    let response = HttpResponse::new(
        if parsed_request.header_line.path == "/"
            || parsed_request.header_line.path == "/index.html"
        {
            StatusCode::Ok
        } else {
            StatusCode::NotFound
        },
    );

    let response = format!("{}", response);

    println!("Response: {}", response);

    socket.try_write(response.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process_socket(socket, addr).await;
        });
    }
}
