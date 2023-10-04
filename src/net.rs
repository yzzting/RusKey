use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::Result;
use tokio::net::TcpStream;
use std::str;
use crate::db::Db;
use crate::cmd;

pub async fn handle_client(mut stream: TcpStream, db: &mut Db) -> Result<()> {
    let mut buffer = [0; 512]; // read up to 512 bytes
    stream.read(&mut buffer).await?; // read bytes from stream

    let command = str::from_utf8(&buffer).unwrap().trim(); // convert bytes to string

    let mut parts = command.split_ascii_whitespace(); // split string into parts

    match cmd::handle_command(&mut parts, db) {
        Ok(response) => stream.write(response.as_bytes()).await?,
        Err(e) => stream.write(e.as_bytes()).await?,
    };

    Ok(())
}
