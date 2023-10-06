use crate::db::Db;
use std::str::SplitAsciiWhitespace;

pub fn handle_command(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
    let cmd = parts.next();
    let arg = parts.next();
    match cmd {
        Some("PING") => {
            match arg {
                Some(arg) => Ok(arg.to_string()),
                None => Ok("PONG".to_string()),
            }
        },

        Some("GET") => {
            if let Some(key) = parts.next() {
                let value = db.get(key).unwrap_or(&db.not_found_message);
                Ok(value.clone())
            } else {
                Err("No such key")
            }
        },
        Some("SET") => {
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                db.set(key.to_string(), value.to_string());
                Ok("OK".to_string())
            } else {
                Err("Invalid arguments")
            }
        },
        _ => Err("Invalid command"),
    }
}