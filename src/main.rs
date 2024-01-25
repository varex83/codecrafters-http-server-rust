mod request;
mod response;

use crate::request::HttpRequest;
use crate::response::{HttpResponse, ResponseBody, StatusCode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
pub enum Command {
    Index,
    Echo(String),
    UserAgent,
    Error,
}

async fn path_handler(path: &str) -> (StatusCode, Command) {
    match path
        .trim_end_matches('/')
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<&str>>()
        .as_slice()
    {
        [""] | ["index"] => (StatusCode::Ok, Command::Index),
        ["echo", value @ ..] => (StatusCode::Ok, Command::Echo(value.join("/").to_string())),
        ["user-agent"] => (StatusCode::Ok, Command::UserAgent),
        _ => (StatusCode::NotFound, Command::Error),
    }
}

async fn process_socket(
    mut socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> anyhow::Result<()> {
    println!("Connection from {}", addr);

    // read the request

    let mut buf = [0; 1024];

    match socket.read(&mut buf).await {
        Ok(0) => {
            println!("Connection closed");
            Ok(())
        }
        Ok(n) => {
            let request = String::from_utf8(buf[0..n].to_vec())?;

            println!("Request: {}", request);

            let parsed_request = HttpRequest::try_from(request)?;

            println!("Parsed request: {:?}", parsed_request);

            // write the response

            let (status_code, command) = path_handler(&parsed_request.header_line.path).await;

            println!("Command: {:?}", command);

            let response = HttpResponse::new(
                status_code,
                match command {
                    Command::Index => None,
                    Command::Echo(value) => Some(ResponseBody::try_from(value.as_str())?),
                    Command::UserAgent => Some(ResponseBody::try_from(
                        parsed_request.get_header("User-Agent").unwrap().as_str(),
                    )?),
                    Command::Error => None,
                },
            );

            let response = format!("{}", response);

            println!("Response: {}", response);

            socket.write_all(response.as_bytes()).await?;

            println!("Response sent");

            Ok(())
        }
        Err(e) => {
            println!("Failed to read from socket; err = {:?}", e);
            Err(e.into())
        }
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
            let _ = process_socket(socket, addr).await;
        });
    }
}
