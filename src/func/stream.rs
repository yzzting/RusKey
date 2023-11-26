use regex::Regex;
use std::time::Duration;
use std::time::Instant;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::net::TcpStream;
use tokio::time::timeout;

const PING_TIMEOUT: u64 = 10; // ping timeout 10 seconds

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn new(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client { stream })
    }

    pub async fn send_command(&mut self, original_command: &str) -> Result<String> {
        let command_message = if original_command.to_lowercase().starts_with("ping") {
            let re = Regex::new(r"(?i)ping").unwrap();
            let command = re.replace(original_command, "").trim().to_string();
            format!("PING {}\r\n", command)
        } else {
            format!("{}\r\n", original_command)
        };
        match self.stream.write_all(command_message.as_bytes()).await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(e);
            }
        }

        let start_time = Instant::now();

        let mut buf = [0; 512];
        match timeout(
            Duration::from_secs(PING_TIMEOUT),
            self.stream.read(&mut buf),
        )
        .await
        {
            Ok(Ok(n)) => {
                if original_command.to_lowercase().starts_with("ping") {
                    let elapsed = start_time.elapsed();
                    println!("Ping time: {} ms", elapsed.as_millis());
                }
                let response = String::from_utf8_lossy(&buf[..n]).to_string();
                Ok(response)
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => {
                println!("Reading response timed out");
                Err(
                    std::io::Error::new(std::io::ErrorKind::TimedOut, "Reading response timed out")
                        .into(),
                )
            }
        }
    }
}
