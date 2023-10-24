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

    fn get(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
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
    }
}

impl Command for StringCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "set" => self.set(parts, db),
            "get" => self.get(parts, db),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}