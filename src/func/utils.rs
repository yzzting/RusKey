use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;

use crate::command_factory::Command;
use crate::db::Db;
use crate::db::DataType;

const EXPIRED: &str = "expired";

pub struct UtilsCommand {
    command: String,
}

impl UtilsCommand {
    pub fn new(command: String) -> UtilsCommand {
        UtilsCommand {
            command,
        }
    }

    fn check_expired(&self, key: Option<&str>, db: &mut Db) -> String {
        let key = match key {
            Some(key) => key,
            None => return "0".to_string(),
        };
        if db.check_expired(key) {
            return "1".to_string();
        }
        "0".to_string()
    }

    fn rename(&self, old_name: Option<&str>, new_name: Option<&str>, type_str: &str, db: &mut Db) -> Result<String, &'static str> {
        let old_name = match old_name {
            Some(old_name) => old_name,
            None => return Err("No such key"),
        };
    
        let new_name = match new_name {
            Some(new_name) => new_name,
            None => return Err("No such key"),
        };
    
        if !db.check_expired(old_name) {
            return Err("No such key");
        }
    
        if db.check_expired(new_name) && type_str == "nx" {
            return Err("New name is exists");
        }
    
        let mut expired_map = match db.get(EXPIRED) {
            Some(DataType::HashMap(expired_map)) => expired_map.clone(),
            None => HashMap::new(),
            _ => HashMap::new(),
        };
    
        // if expired_map contains old_name, insert new_name
        if expired_map.contains_key(old_name) {
            let old_expired = match expired_map.get(old_name) {
                Some(old_expired) => old_expired,
                None => return Err("No such key"),
            };
    
            expired_map.insert(new_name.to_string(), old_expired.to_string());
            db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));
        }
    
        let value = match db.get(old_name) {
            Some(value) => value.clone(),
            None => return Err("No such key"),
        };
    
        db.set(new_name.to_string(), value);
    
        if !db.delete(old_name) {
            return Err("Failed to delete old key");
        }
    
        Ok("OK".to_string())
    }

    fn check_type(&self, key: Option<&str>, db: &mut Db) -> String {
        let key = match key {
            Some(key) => key,
            None => return "none".to_string(),
        };
        if !db.check_expired(key) {
            return "none".to_string();
        }
        match db.get(key) {
            Some(DataType::String(_)) => "string".to_string(),
            // Some(DataType::List(_)) => "list".to_string(),
            // Some(DataType::Set(_)) => "set".to_string(),
            Some(DataType::HashMap(_)) => "hash".to_string(),
            Some(DataType::ZSet(_)) => "zset".to_string(),
            None => "none".to_string(),
        }
    }

    fn randomkey(&self, db: &mut Db) -> String {
        match db.randomkey() {
            Some(key) => key,
            None => "nil".to_string(),
        }
    }

    fn del_key(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let mut count = 0;
        while let Some(key) = parts.next() {
            if db.delete(key) {
                count += 1;
            }
        }
        count.to_string()
    }
}

impl Command for UtilsCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "exists" => Ok(self.check_expired(parts.next(), db)),
            "rename" => {
                let old_name = parts.next();
                let new_name = parts.next();
                let rename = self.rename(old_name, new_name, "", db);
                rename
            },
            "renamenx" => {
                let old_name = parts.next();
                let new_name = parts.next();
                let rename = self.rename(old_name, new_name, "nx", db);
                rename
            },
            "randomkey" => {
                Ok(self.randomkey(db))
            },
            "del" => {
                Ok(self.del_key(parts, db))
            },
            "type" => {
                Ok(self.check_type(parts.next(), db))
            },
            _ => Err("UtilsCommand Error: Command not found"),
        }
    }
}