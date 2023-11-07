use std::process;
use std::collections::HashSet;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::Store;
use crate::func::stream::Client;

const COMMANDS: [&str; 18] = ["ping", "expired", "expireat", "pexpireat", "ttl", "pttl", "persist", "exists", "del", "rename", "renamenx", "type", "randomkey", "get", "set", "getrange", "hmset", "hgetall"];

async fn send_command(command: &str, state: &Store) {
    let mut client = Client::new(&state.url).await.unwrap();
    match client.send_command(command).await {
        Ok(response) => {
            println!("{:?}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

pub async fn read_line(state: &Store) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let commands = COMMANDS.iter().cloned().collect::<HashSet<_>>();
    loop {
        match rl.readline(format!("{} RusKey >", state.url).as_str()) {
            Ok(line) => {
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: {:?}", err);
                        return Err(err);
                    }
                };

                // handle input
                let lowercase_line = line.trim().to_lowercase();
                let parts: Vec<&str> = lowercase_line.split_whitespace().collect();
                match parts.get(0) {
                    Some(&"quit") | Some(&"exit") => {
                        println!("Exiting RusKey");
                        process::exit(0);
                    }
                    Some(&"config") => {
                        match parts.get(1) {
                            Some(&"get") | Some(&"set") => {
                                send_command(&parts.join(" "), &state).await;
                            }
                            _ => {
                                println!("Read Config Invalid command");
                            }
                        }
                    }
                    Some(command) if commands.contains(command) => {
                        send_command(&parts.join(" "), &state).await;
                    }
                    _ => {
                        println!("Read Invalid command");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Command-C");
                process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                println!("Command-D");
                process::exit(0);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(err);
            }
        }
    }
}
