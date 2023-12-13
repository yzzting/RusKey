use std::io::Result;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use std::sync::{Arc, Mutex};
use crate::cmd;
use rus_key_factory::command_factory::CommandFactory;
use rus_key_trait::db_trait::Db;
use std::str;
use tokio::net::TcpStream;

pub async fn handle_client(mut stream: TcpStream, db: &mut dyn Db) -> Result<()> {
    let mut buffer = [0; 512]; // read up to 512 bytes
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
        let factory = CommandFactory::new();
        match cmd::handle_command(&mut parts, db, &factory) {
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
