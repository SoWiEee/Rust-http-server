use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.expect("[X] Failed to read data from socket");
            let request = str::from_utf8(&buf[..n]).expect("[X] Failed to convert buffer to string");

            // 解析 HTTP 請求
            let lines: Vec<&str> = request.lines().collect();
            let request_line = lines[0];
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            let method = parts[0];
            let path = parts[1];

            println!("Method: {}", method);
            println!("Path: {}", path);

            for line in &lines[1..] {
                if line.contains("Host:") {
                    println!("Host: {}", line);
                } else if line.contains("User-Agent:") {
                    println!("User-Agent: {}", line);
                } else if line.contains("Content-Length:") {
                    println!("Content-Length: {}", line);
                }
            }

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
            socket.write_all(response.as_bytes()).await.expect("failed to write data to socket");
        });
    }
}
