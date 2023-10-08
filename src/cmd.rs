use crate::db::Db;
use crate::db::DataType;
use std::collections::BTreeMap;
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
        // Hash Map

        "hmset" => {
            let key = match parts.next() {
                Some(key) => key,
                None => return Err("Key not specified"),
            };
            let mut btree_map = BTreeMap::new();
            while let Some(field) = parts.next() {
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("Value not specified"),
                };
                btree_map.insert(field.to_string(), value.to_string());
            }
            db.set(key.to_string(), DataType::HashMap(btree_map));
            Ok("OK".to_string())
        }

        "hgetall" => {
            let key = match parts.next() {
                Some(key) => key,
                None => return Err("Key not specified"),
            };
            match db.get(key) {
                Some(DataType::HashMap(btree_map)) => {
                    let mut result = String::new();
                    for (field, value) in btree_map {
                        result.push_str(&format!("{}: {} ", field, value));
                    }
                    Ok(result.trim().to_string())
                }
                _ => Err("No such key or wrong data type"),
            }
        }
        _ => Err("Invalid command!"),
    }
}