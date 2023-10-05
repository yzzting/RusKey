use std::process;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::func::ping::ping;

pub async fn read_line(url: &str) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline(format!("{} RusKey >", url).as_str()) {
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
                    Some(&"ping") => {
                        let command = parts.join(" ");
                       ping( &command, &url).await;
                     }
                    _ => {}
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
