use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("{}", "Server is running at http://localhost:3000".blue().bold());
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(n) => {
                    let request = str::from_utf8(&buf[..n]).expect("[X] Failed to convert buffer to string");

                    // 解析 HTTP 請求
                    let lines: Vec<&str> = request.lines().collect();
                    let request_line = lines[0];
                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                    let method = parts[0];
                    let path = parts[1];

                    println!("{}", "Method: ".green().bold());
                    println!("{}", method);
                    println!("{}", "Path: ".green().bold());
                    println!("{}", path);

                    for line in &lines[1..] {
                        if line.contains("Host:") {
                            println!("{}", "Host: ".green().bold());
                            println!("{}", line);
                        } else if line.contains("User-Agent:") {
                            println!("{}", "User-Agent: ".green().bold());
                            println!("{}", line);
                        } else if line.contains("Content-Length:") {
                            println!("{}", "Content-Length: ".green().bold());
                            println!("{}", line);
                        }
                    }

                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
                    match socket.write_all(response.as_bytes()).await {
                        Ok(_) => println!("{}", "[V] Response sent successfully!".green()),
                        Err(e) => eprintln!("{}", format!("[X] Failed to send response: {}", e).red()),
                    }
                },
                Err(e) => eprintln!("{}", format!("[X] Failed to read data from socket: {}", e).red()),
            }
        });
    }
}
