mod net;
mod db;
mod cmd;
mod args;
mod read_line;

use std::net::TcpListener;
use crate::db::Db;
use crate::net::handle_client;
use clap::Parser;
use crate::args::Opt;
use crate::read_line::read_line;

fn main() {
    let opt = Opt::parse();
    let mut db = Db::new();
    let url = format!("{}:{}", opt.host, opt.port);
    let listener = TcpListener::bind(url.clone()).unwrap();

    println!("rus key start {}:{}", opt.host, opt.port);

    if let Err(e) = read_line(&url) {
        println!("Error: {:?}", e);
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        if let Err(e) = handle_client(stream, &mut db) {
            println!("Failed to handle client: {}", e);
        }
    }
}