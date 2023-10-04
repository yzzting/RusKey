mod net;
mod db;
mod cmd;

use std::net::TcpListener;
use crate::db::Db;
use crate::net::handle_client;

fn main() {
    let mut db = Db::new();
    let listener = TcpListener::bind("127.0.0.1:16379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        if let Err(e) = handle_client(stream, &mut db) {
            println!("Failed to handle client: {}", e);
        }
    }
    println!("rus key start!");
}
