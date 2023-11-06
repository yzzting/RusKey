use std::collections::BTreeMap;
use std::str::SplitAsciiWhitespace;

use crate::command_factory::Command;
use crate::db::Db;
use crate::db::DataType;
use crate::func::expired::get_key_expired;

pub struct HashMapCommand {
    command: String,
}

impl HashMapCommand {
    pub fn new(command: String) -> HashMapCommand {
        HashMapCommand {
            command,
        }
    }

    fn hmset(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
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
        db.set(key.to_string(), DataType::ZSet(btree_map));
        Ok("OK".to_string())
    }

    fn hgetall(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
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
            Some(DataType::ZSet(btree_map)) => {
                let mut result = String::new();
                for (field, value) in btree_map {
                    result.push_str(&format!("{}: {} ", field, value));
                }
                Ok(result.trim().to_string())
            }
            _ => Err("There is no such key, the key is expired, or the data type is incorrect"),
        }
    }
}

impl Command for HashMapCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "hmset" => self.hmset(parts, db),
            "hgetall" => self.hgetall(parts, db),
            _ => Err("HashMapCommand Error: Command not found"),
        }
    }
}
