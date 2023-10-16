use crate::db::Db;
use crate::db::DataType;
use std::collections::BTreeMap;
use std::str::SplitAsciiWhitespace;

use crate::func::config::handle_config;
use crate::func::expired::expired::{handle_expired, get_key_expired, handle_ttl};

pub fn handle_command(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
    let cmd = match parts.next() {
        Some(cmd) => cmd.to_lowercase(),
        None => return Err("No command"),
    };
    if cmd == "config" {
        return handle_config(parts, db);
    }
    match cmd.as_str() {
        // Connection
        "ping" => {
            let arg = parts.next();
            match arg {
                Some(arg) => Ok(arg.to_string()),
                None => Ok("PONG".to_string()),
            }
        },
        // Expired
        "expired" => {
            let key = parts.next();
            let value = parts.next();
            return handle_expired(key, value, db);
        },
        // TTL
        "ttl" => {
            let key = parts.next();
            return handle_ttl(key, db)
        },
        // String
        "get" => {
            let key = parts.next();
            if let Some(key) = key {
                // check expired
                let expired = get_key_expired(Some(key), db);
                if !expired.is_empty() && expired != "nil" {
                    return Err("There is no such key, the key is expired, or the data type is incorrect");
                }
                if expired == "nil" {
                    return Err("nil");
                }
                match db.get(key) {
                    Some(DataType::String(value)) => Ok(value.clone()),
                    _ => Err("There is no such key, the key is expired, or the data type is incorrect"),
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
            db.set(key.to_string(), DataType::BTreeMap(btree_map));
            Ok("OK".to_string())
        }

        "hgetall" => {
            let key = match parts.next() {
                Some(key) => key,
                None => return Err("Key not specified"),
            };
            // check expired
            let expired = get_key_expired(Some(key), db);
            if !expired.is_empty() && expired != "nil" {
                return Err("There is no such key, the key is expired, or the data type is incorrect");
            }
            if expired == "nil" {
                return Err("nil");
            }
            match db.get(key) {
                Some(DataType::HashMap(btree_map)) => {
                    let mut result = String::new();
                    for (field, value) in btree_map {
                        result.push_str(&format!("{}: {} ", field, value));
                    }
                    Ok(result.trim().to_string())
                }
                _ => Err("There is no such key, the key is expired, or the data type is incorrect"),
            }
        }
        _ => Err("Invalid command!"),
    }
}