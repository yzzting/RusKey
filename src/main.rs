mod net;
mod db;
mod cmd;
mod args;
mod read_line;
mod func;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use crate::db::Db;
use crate::net::handle_client;
use clap::Parser;
use crate::args::Opt;
use crate::read_line::read_line;

pub struct Store {
    url: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let url = format!("{}:{}", opt.host, opt.port);
    let listener = TcpListener::bind(url.clone()).await.unwrap();

    println!("rus key start {}:{}", opt.host, opt.port);

    let state = Store { url };
    
    let db = Arc::new(Mutex::new(Db::new()));

    tokio::spawn(async move {
        if let Err(e) = read_line(&state).await {
            println!("Error: {:?}", e);
        }
    });

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let db = Arc::clone(&db);
        tokio::spawn(async move {
            let mut db_lock = db.lock().await;
            if let Err(e) = handle_client(stream, &mut *db_lock).await {
                println!("Error: {:?}", e);
            }
        });
    }
}