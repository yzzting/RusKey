use std::cmp::min;
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

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let key = parts.next();
        let mut value = parts.next().map(|s| s.to_string());
        if let Some(ref mut value_str) = value {
            if value_str.starts_with('"') && !value_str.ends_with('"') {
                while let Some(part) = parts.next() {
                    value_str.push_str(" ");
                    value_str.push_str(part);
                    if part.ends_with('"') {
                        break;
                    }
                }
            }
        }
        let value = value.map(|s| s.trim_matches('"').to_string());
        if let (Some(key), Some(value)) = (key, value) {
            db.set(key.to_string(), DataType::String(value.to_string()));
            "OK".to_string()
        } else {
            "Set Error: Key or value not specified".to_string()
        }
    }

    fn get(&self, key: &str, db: &mut Db) -> String {
        // check expired
        let expired = get_key_expired(Some(key), db);
        if !expired.is_empty() && expired != "nil" {
            return "There is no such key, the key is expired, or the data type is incorrect".to_string();
        }
        if expired == "nil" {
            return "nil".to_string();
        }
        match db.get(key) {
            Some(DataType::String(value)) => value.clone(),
            _ => "There is no such key, the key is expired, or the data type is incorrect".to_string(),
        }
    }

    fn get_range(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let key = parts.next().unwrap();
        let start = match parts.next() {
            Some(start_str) => match start_str.parse::<isize>() {
                Ok(start) => start,
                Err(_) => return "GetRange Error: Invalid start value".to_string(),
            },
            None => return "GetRange Error: Start not specified".to_string(),
        };
        let end = match parts.next() {
            Some(end_str) => match end_str.parse::<isize>() {
                Ok(end) => end,
                Err(_) => return "GetRange Error: Invalid end value".to_string(),
            },
            None => return "GetRange Error: End not specified".to_string(),
        };

        let key_value = self.get(key, db);
        if key_value == "There is no such key, the key is expired, or the data type is incorrect" {
            return "".to_string();
        }
        return StringCommand::slice_from_end(&key_value, start, end);
    }

    fn slice_from_end(str: &str, start: isize, end: isize) -> String {
        let char_vec: Vec<char> = str.chars().collect();
        let char_vec_len = char_vec.len() as isize;
        let start  = if start < 0 {
            let pos_start = char_vec_len + start;
            if pos_start < 0 {
                0
            } else {
                pos_start as usize
            }
        } else {
            start as usize
        };

        let end = if end < 0 {
            let pos_end = char_vec_len + end + 1;
            if pos_end < 0 {
                1
            } else {
                pos_end as usize
            }
        } else {            
            if end >= char_vec_len {
                char_vec_len as usize
            } else {
                min((end + 1) as usize, char_vec_len as usize)
            }
        };
        if start > end {
            "".to_string();
        }
        char_vec[start..end].iter().collect::<String>()
    }
}

impl Command for StringCommand {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        match self.command.as_str() {
            "set" => Ok(self.set(parts, db)),
            "get" => Ok(self.get(parts.next().unwrap(), db)),
            "getrange" => Ok(self.get_range(parts, db)),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}