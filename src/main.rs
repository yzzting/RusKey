mod net;
mod db;
mod cmd;
mod args;
mod read_line;

use tokio::net::TcpListener;
use crate::db::Db;
use crate::net::handle_client;
use clap::Parser;
use crate::args::Opt;
use crate::read_line::read_line;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let url = format!("{}:{}", opt.host, opt.port);
    let listener = TcpListener::bind(url.clone()).await.unwrap();

    println!("rus key start {}:{}", opt.host, opt.port);

    if let Err(e) = read_line(&url) {
        println!("Error: {:?}", e);
    }

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let mut db = Db::new();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, &mut db).await {
                println!("Error: {:?}", e);
            }
        });
    }
}