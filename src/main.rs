mod net;
mod db;
mod cmd;
mod args;

use std::net::TcpListener;
use crate::db::Db;
use crate::net::handle_client;
use clap::Parser;
use crate::args::Opt;

fn main() {
    let opt = Opt::parse();
    let mut db = Db::new();
    let listener = TcpListener::bind(format!("{}:{}", opt.host, opt.port)).unwrap();

    println!("rus key start {}:{}", opt.host, opt.port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        if let Err(e) = handle_client(stream, &mut db) {
            println!("Failed to handle client: {}", e);
        }
    }
}