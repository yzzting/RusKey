use std::str::SplitAsciiWhitespace;
use crate::utils::{get_current_time, get_expired_map};
use crate::handle_expired::handle_expired;
use crate::del_key_expired::del_key_expired;
use crate::handle_ttl::handle_ttl;

use rus_key_trait::command_trait::Command;
use rus_key_db::db::Db;

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
            Ok(n) if n > current_time => n.to_string(),
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
        ExpiredCommand { command }
    }
}

impl Command for ExpiredCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "expired" => {
                let key = parts.next();
                let value = parts.next();
                let expired = handle_expired(key, value, "", db);
                expired
            }
            "expireat" => {
                let key = parts.next();
                let value = parts.next();
                let expireat = handle_expired(key, value, "at", db);
                expireat
            }
            "pexpireat" => {
                let key = parts.next();
                let value = parts.next();
                let pexpireat = handle_expired(key, value, "p", db);
                pexpireat
            }
            "ttl" => {
                let key = parts.next();
                let ttl = handle_ttl(key, "", db);
                Ok(ttl.to_string())
            }
            "pttl" => {
                let key = parts.next();
                let ttl = handle_ttl(key, "p", db);
                Ok(ttl.to_string())
            }
            "persist" => {
                let key = parts.next();
                let persist = del_key_expired(key, db);
                Ok(persist.to_string())
            }
            _ => Err("ExpiredCommand Error: Command not found"),
        }
    }
}
