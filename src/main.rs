use rus_key_lib::args::Opt;
use rus_key_db::db::{Db, DataType};
use rus_key_lib::init::{Config, init, Store};
use rus_key_lib::net::handle_client;
use rus_key_lib::read_line::read_line;
use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Db::new()));
    // init config
    let config_map = init();
    println!("config: {:?}", config_map);
    db.lock().await.set(
        "ruskey_config".to_string(),
        DataType::ZSet(config_map.clone()),
    );
    // parse args priority command line > config file
    let opt = Opt::parse();
    let config = Config::new(opt, config_map);
    let host = config
        .get("host")
        .unwrap_or_else(|| String::from("127.0.0.1"));
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
