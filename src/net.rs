use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::Result;
use std::io::{Error, ErrorKind};
// use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use std::str;
use crate::db::Db;
use crate::cmd;

pub async fn handle_client(mut stream: TcpStream, db: &mut Db) -> Result<()> {
    let mut buffer = [0; 512]; // read up to 512 bytes
    // let mut stream = stream.lock().unwrap();
    loop {
        let bytes_read = stream.read(&mut buffer).await.map_err(|e| {
            println!("Error: {:?}", e);
            Error::new(ErrorKind::Other, "Failed to read from socket")
        })?;

        // if read returned 0, client has closed the connection
        if bytes_read == 0 {
            break;
        }
        let command = str::from_utf8(&buffer[..bytes_read]).unwrap().trim(); // convert bytes to string

        let mut parts = command.split_ascii_whitespace(); // split string into parts

        match cmd::handle_command(&mut parts, db) {
            Ok(response) => stream.write(response.as_bytes()).await.map_err(|e| {
                println!("Error: {:?}", e);
                Error::new(ErrorKind::Other, "Failed to write to socket")
            })?,
            Err(e) => stream.write(e.as_bytes()).await.map_err(|e| {
                println!("Error: {:?}", e);
                Error::new(ErrorKind::Other, "Failed to write to socket")
            })?,
        };
    }

    Ok(())
}
