use crate::db::Db;
use crate::db::DataType;
use std::str::SplitAsciiWhitespace;

pub fn handle_command(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
    let cmd = match parts.next() {
        Some(cmd) => cmd.to_lowercase(),
        None => return Err("No command"),
    };
    match cmd.as_str() {
        // Connection
        "ping" => {
            let arg = parts.next();
            match arg {
                Some(arg) => Ok(arg.to_string()),
                None => Ok("PONG".to_string()),
            }
        },
        // String
        "get" => {
            let key = parts.next();
            if let Some(key) = key {
                match db.get(key) {
                    Some(DataType::String(value)) => Ok(value.clone()),
                    _ => Err("No such key or wrong data type"),
                }
            } else {
                Err("No such key")
            }
        },

        "set" => {
            let key = parts.next();
            let value = parts.next();
            if let (Some(key), Some(value)) = (key, value) {
                db.set(key.to_string(), DataType::String(value.to_string()));
                Ok("OK".to_string())
            } else {
                Err("Set Error: Key or value not specified")
            }
        },
        _ => Err("Invalid command!"),
    }
}