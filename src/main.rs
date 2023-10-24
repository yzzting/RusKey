mod net;
mod db;
mod cmd;
mod command_factory;
mod args;
mod read_line;
mod func;
mod init;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use crate::db::Db;
use crate::db::DataType;
use crate::net::handle_client;
use clap::Parser;
use crate::args::Opt;
use crate::init::Config;
use crate::read_line::read_line;

pub struct Store {
    url: String,
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Db::new()));
    // init config
    let config_map = init::init();
    println!("config: {:?}", config_map);
    db.lock().await.set("ruskey_config".to_string(), DataType::ZSet(config_map.clone()));
    // parse args priority command line > config file
    let opt = Opt::parse();
    let config = Config::new(opt, config_map);
    let host = config.get("host").unwrap_or_else(|| String::from("127.0.0.1"));
    let port = config.get("port").unwrap_or_else(|| String::from("16379"));
    let url = format!("{}:{}", host, port);
    println!("rus key start {}:{}", host, port);
    let listener = TcpListener::bind(url.clone()).await.unwrap();

    let state = Store { url };
    
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