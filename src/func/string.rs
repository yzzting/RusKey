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
        let mut value_parts = Vec::new();
        while let Some(part) = parts.next() {
            if part.starts_with('"') && !part.ends_with('"') {
                value_parts.push(part[1..].to_string());
                while let Some(part) = parts.next() {
                    value_parts.push(part.to_string());
                    if part.ends_with('"') {
                        break;
                    }
                }
            } else {
                value_parts.push(part.to_string());
            }
        }
        let value = Some(value_parts.join(" ").trim_end_matches('"').to_string());
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
            Some(start_str) => match start_str.parse::<isize>() {
                Ok(start) => start,
                Err(_) => return Err("GetRange Error: Invalid start value"),
            },
            None => return Err("GetRange Error: Start not specified"),
        };
        let end = match parts.next() {
            Some(end_str) => match end_str.parse::<isize>() {
                Ok(end) => end,
                Err(_) => return Err("GetRange Error: Invalid end value"),
            },
            None => return Err("GetRange Error: End not specified"),
        };

        let key_value = self.get(key, db).unwrap();
        let key_value_splice = StringCommand::slice_from_end(&key_value, start, end);

        Ok(key_value_splice.to_string())
    }

    fn slice_from_end(str: &str, start: isize, end: isize) -> String {
        println!("str: {}", str);
        let char_vec: Vec<char> = str.chars().collect();

        let start  = if start < 0 {
            (char_vec.len() as isize + start) as usize
        } else {
            start as usize
        };

        let end = if end < 0 || end < char_vec.len() as isize - 1 {
            (end + 1) as usize
        } else {
            end as usize
        };
        println!("start: {}", start);
        println!("end: {}", end);
        char_vec[start..end].iter().collect::<String>()
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