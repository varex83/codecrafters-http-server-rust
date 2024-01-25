mod files;
mod http;
mod request;
mod response;

use crate::files::FILES_ROOT_DIR;
use crate::http::process_socket;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    FILES_ROOT_DIR
        .set(if args.len() > 2 && args[1] == "--directory" {
            args[2].clone()
        } else {
            "./files".to_string()
        })
        .unwrap();

    println!("Files root dir: {}", FILES_ROOT_DIR.get().unwrap());

    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let _ = process_socket(socket, addr).await;
        });
    }
}
