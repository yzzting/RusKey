use std::process;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn read_line(url: &str) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        match rl.readline(format!("{} RusKey >", url).as_str()) {
            Ok(line) => {
                rl.save_history("history.txt")?;
                println!("input: {}", line);
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: {:?}", err);
                        return Err(err);
                    }
                };

                // handle input
                match line.trim() {
                    "quit" | "exit" => {
                        println!("Exiting RusKey");
                        process::exit(0);
                    }
                    "ping" => {
                        println!("pong");
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
