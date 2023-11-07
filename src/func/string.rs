use std::str::SplitAsciiWhitespace;
use crate::db::Db;
use crate::db::DataType;
use crate::command_factory::Command;
use crate::func::expired::get_key_expired;

pub struct StringCommand {
    command: String,
}

impl StringCommand {
    pub fn new(command: String) -> StringCommand {
        StringCommand {
            command,
        }
    }

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        let key = parts.next();
        let value = parts.next();
        if let (Some(key), Some(value)) = (key, value) {
            db.set(key.to_string(), DataType::String(value.to_string()));
            Ok("OK".to_string())
        } else {
            Err("Set Error: Key or value not specified")
        }
    }

    fn get(&self, key: &str, db: &mut Db) -> Result<String, &'static str> {
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
    }

    fn get_range(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        let key = parts.next().unwrap();
        let start = match parts.next() {
            Some(start_str) => match start_str.parse::<usize>() {
                Ok(start) => start,
                Err(_) => return Err("GetRange Error: Invalid start value"),
            },
            None => return Err("GetRange Error: Start not specified"),
        };
        let end = match parts.next() {
            Some(end_str) => match end_str.parse::<usize>() {
                Ok(end) => end + 1,
                Err(_) => return Err("GetRange Error: Invalid end value"),
            },
            None => return Err("GetRange Error: End not specified"),
        };

        let key_value = self.get(key, db).unwrap();
        let key_value_splice = &key_value[start..end];

        Ok(key_value_splice.to_string())
    }
}

impl Command for StringCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "set" => self.set(parts, db),
            "get" => self.get(parts.next().unwrap(), db),
            "getrange" => self.get_range(parts, db),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}