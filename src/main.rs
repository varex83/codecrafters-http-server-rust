use std::fmt::{Display, Formatter};
// Uncomment this block to pass the first stage
use anyhow::bail;
use anyhow::Result;
use tokio::net::TcpListener;

#[derive(Debug)]
pub enum StatusCode {
    Ok,
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StatusCode::Ok => {
                    "200"
                }
            }
        )
    }
}

impl TryFrom<&str> for StatusCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "200" => Ok(Self::Ok),
            _ => bail!("invalid status code provided"),
        }
    }
}

#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: StatusCode,
}

impl HttpResponse {
    pub fn new(status: StatusCode) -> Self {
        HttpResponse { status }
    }
}

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

async fn process_socket(socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("Connection from {}", addr);

    let response = HttpResponse::new(StatusCode::Ok);

    socket.try_write(response.to_string().as_bytes()).unwrap();
}
