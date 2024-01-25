use crate::files::FILES_ROOT_DIR;
use crate::request::HttpRequest;
use crate::response::{HttpResponse, ResponseBody, StatusCode};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum Command {
    Index,
    Echo(String),
    UserAgent,
    Files(String),
    Error,
}

pub async fn path_handler(path: &str) -> (StatusCode, Command) {
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
        ["files", filename] => (StatusCode::Ok, Command::Files(filename.to_string())),
        _ => (StatusCode::NotFound, Command::Error),
    }
}

pub async fn process_socket(
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

            let response = match command {
                Command::Index => HttpResponse::new(status_code, None),
                Command::Echo(value) => {
                    HttpResponse::new(status_code, Some(ResponseBody::try_from(value.as_str())?))
                }
                Command::UserAgent => HttpResponse::new(
                    status_code,
                    Some(ResponseBody::try_from(
                        parsed_request.get_header("User-Agent").unwrap().as_str(),
                    )?),
                ),
                Command::Files(filename) => {
                    let files_prefix = FILES_ROOT_DIR.get().unwrap();
                    let content =
                        tokio::fs::read_to_string(Path::new(files_prefix).join(filename)).await;

                    match content {
                        Ok(content) => HttpResponse::new(
                            status_code,
                            Some(
                                ResponseBody::try_from(content.as_str())?
                                    .with_content_type("application/octet-stream"),
                            ),
                        ),
                        Err(_e) => HttpResponse::new(StatusCode::NotFound, None),
                    }
                }
                Command::Error => HttpResponse::new(status_code, None),
            };

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
