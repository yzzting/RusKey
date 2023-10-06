use tokio::io::Result;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::time::Duration;
use std::time::Instant;
use regex::Regex;

use crate::Store;

const PING_TIMEOUT: u64 = 10; // ping timeout 10 seconds

async fn send_ping(original_command: &str, addr: &str) -> Result<String> {
    let mut stream: TcpStream = timeout(Duration::from_secs(PING_TIMEOUT), TcpStream::connect(addr)).await??;
    let re = Regex::new(r"(?i)ping").unwrap();
    let command = re.replace(original_command, "").trim().to_string();
    let ping_message = format!("PING {}\r\n", command);
    match stream.write_all(ping_message.as_bytes()).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e);
        }
    }

    let start_time = Instant::now();

    let mut buf = [0; 512];
    match timeout(Duration::from_secs(PING_TIMEOUT), stream.read(&mut buf)).await {
        Ok(Ok(n)) => {
            let elapsed = start_time.elapsed();
            println!("Ping time: {} ms", elapsed.as_millis());
            let response = String::from_utf8_lossy(&buf[..n]).to_string();
            Ok(response)
        },
        Ok(Err(e)) => {
            Err(e.into())
        },
        Err(_) => {
            println!("Reading response timed out");
            Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Reading response timed out").into())
        },
    }
}

pub async fn ping(command: &str, state: &Store) {
    match send_ping(command, &state.url).await {
        Ok(response) => {
            println!("Received: {}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}