use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn read_line(url: &str) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut input = String::new();
    loop {
        match rl.readline(format!("{} RusKey >", url).as_str()) {
            Ok(line) => {
            println!("input: {}", line);
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: {:?}", err);
                        return Err(err);
                    },
                };
                input = line;
            },
            Err(ReadlineError::Interrupted) => {
                println!("Command-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Command-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(err);
            },
        }
    }

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt")?;

    Ok(input)
}