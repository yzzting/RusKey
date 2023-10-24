use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;

use crate::command_factory::Command;
use crate::db::Db;
use crate::db::DataType;

const EXPIRED: &str = "expired";

fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp_millis();
    timestamp
}

// splice current time and expired time
fn splice_time(expired: i64) -> i64 {
    let current_time = get_current_time();
    let expired_time = current_time + expired;
    expired_time
}

fn get_expired_map(db: &mut Db) -> HashMap<String, String> {
    let expired_map = match db.get(EXPIRED) {
        Some(DataType::HashMap(expired_map)) => expired_map.clone(),
        None => HashMap::new(),
        _ => HashMap::new(),
    };
    expired_map
}

pub fn get_key_expired(key: Option<&str>, db: &mut Db) -> String {
    let key = match key {
        Some(key) => key,
        None => return "No such key".to_string(),
    };

    if !db.check_expired(key) {
        return "No such key".to_string();
    }

    let expired_map = get_expired_map(db);

    let current_time = get_current_time();
    let expired_time = match expired_map.get(key) {
        Some(expired_time) => match expired_time.parse::<i64>() {
            Ok(n) if n > current_time => "".to_string(),
            _ => {
                db.delete(key);
                return "nil".to_string();
            }
        },
        None => "".to_string(),
    };
    expired_time
}

pub struct ExpiredCommand {
    command: String,
}

impl ExpiredCommand {
    pub fn new(command: String) -> ExpiredCommand {
        ExpiredCommand {
            command,
        }
    }
    
    fn handle_expired(&self, key: Option<&str>, value: Option<&str>, type_str: &str, db: &mut Db) -> Result<String, &'static str> {
        let key = match key {
            Some(key) => key,
            None => return Err("No such key"),
        };
    
        if !db.check_expired(key) {
            return Err("No such key");
        }
    
        let (flag_number, multiplier) = match type_str {
            "" => (0, 1),
            "at" => (get_current_time() / 1000, 1000),
            "p" => (get_current_time(), 1),
            _ => return Err("Invalid type"),
        };
    
        let value = match value {
            Some(v) => match v.parse::<i64>() {
                Ok(n) if n > flag_number => n * multiplier,
                _ => return Err("Invalid value"),
            },
            None => return Err("Invalid value"),
        };
    
        let mut expired_map = get_expired_map(db);
    
        let expired_time = if type_str == "" {
            splice_time(value * 1000)
        } else {
            value
        };
    
        expired_map.insert(key.to_string(), expired_time.to_string());
        db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));
    
        Ok("OK".to_string())
    }

    fn del_key_expired(&self, key: Option<&str>, db: &mut Db) -> String {
        let key = match key {
            Some(key) => key,
            None => return "0".to_string(),
        };
    
        let mut expired_map = get_expired_map(db);
    
        if !expired_map.contains_key(key) {
            return "0".to_string();
        }
    
        expired_map.remove(key);
        db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));
    
        "1".to_string()
    }

    fn handle_ttl(&self, key: Option<&str>, type_str: &str, db: &mut Db) -> i64 {
        let key = match key {
            Some(key) => key,
            None => return -2,
        };
    
        if !db.check_expired(key) {
            return -2;
        }
    
        let expired_map = get_expired_map(db);
    
        let current_time = get_current_time();
        let expired_time = match expired_map.get(key) {
            Some(expired_time) => match expired_time.parse::<i64>() {
                Ok(n) if n > current_time => n,
                _ => {
                    db.delete(key);
                    return -2;
                }
            },
            None => return -2,
        };
        let multiplier = match type_str {
            "" => 1,
            "p" => 1000,
            _ => 1,
        };
        ((expired_time - current_time) / 1000) * multiplier
    }
}

impl Command for ExpiredCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "expired" => {
                let key = parts.next();
                let value = parts.next();
                let expired = self.handle_expired(key, value, "", db);
                expired
            },
            "expireat" => {
                let key = parts.next();
                let value = parts.next();
                let expireat = self.handle_expired(key, value, "at", db);
                expireat
            },
            "pexpireat" => {
                let key = parts.next();
                let value = parts.next();
                let pexpireat = self.handle_expired(key, value, "p", db);
                pexpireat
            },
            "ttl" => {
                let key = parts.next();
                let ttl = self.handle_ttl(key, "", db);
                Ok(ttl.to_string())
            },
            "pttl" => {
                let key = parts.next();
                let ttl = self.handle_ttl(key, "p", db);
                Ok(ttl.to_string())
            },
            "persist" => {
                let key = parts.next();
                let persist = self.del_key_expired(key, db);
                Ok(persist.to_string())
            },
            _ => Err("ExpiredCommand Error: Command not found"),
        }
    }
}