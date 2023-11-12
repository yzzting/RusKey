use std::cmp::min;
use std::str::SplitAsciiWhitespace;
use crate::db::Db;
use crate::db::DataType;
use crate::command_factory::Command;
use crate::func::expired::get_key_expired;
use crate::func::expired::ExpiredCommand;

enum SetError {
    InvalidExpiredTime,
    KeyOfValueNotSpecified,
}

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
        let expired_command = ExpiredCommand::new("expired".to_string());
        let key = parts.next();
        // Parse value
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
        let mut expired_time: Option<i64> = None;
        let mut error_str: Option<SetError> = None;
        // String Vec to store extra args
        let mut extra_args: Vec<String> = Vec::new();
        // After parsing value, parse all remaining args
        while let Some(arg) = parts.next() {
            println!("arg: {}", arg);
            let lower_arg = arg.to_lowercase();
            match lower_arg.as_str() {
                "ex" => {
                    extra_args.push(lower_arg.to_string());
                    if let Some(seconds_str) = parts.next() {
                        let seconds = seconds_str.parse::<i64>().unwrap();
                        expired_time = Some(seconds);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                },
                "px" => {
                    extra_args.push(lower_arg.to_string());
                    if let Some(milliseconds_str) = parts.next() {
                        let milliseconds = milliseconds_str.parse::<i64>().unwrap();
                        expired_time = Some((milliseconds + 999) / 1000);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                },
                "exat" => {
                    extra_args.push(lower_arg.to_string());
                },
                "pxat" => {
                    extra_args.push(lower_arg.to_string());
                },
                _ => {},
            }
        }

        // ex/px and exat/pxat cannot exist simultaneously
        if extra_args.contains(&"ex".to_string()) && extra_args.contains(&"exat".to_string()) {
            return "Set Error: Invalid expired time in set".to_string();
        }

        if extra_args.contains(&"px".to_string()) && extra_args.contains(&"pxat".to_string()) {
            return "Set Error: Invalid expired time in set".to_string();
        }

        if let (Some(key), Some(value)) = (key, value) {
            db.set(key.to_string(), DataType::String(value.to_string()));
            // set expired time
            if let Some(expired_time) = expired_time {
                let expired_command_str = format!("{} {}", key, expired_time);
                let mut expired_parts = expired_command_str.split_ascii_whitespace();
                let expired_result = expired_command.execute(&mut expired_parts, db).unwrap();
                if expired_result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
        } else {
            error_str = Some(SetError::KeyOfValueNotSpecified);
        }

        if let Some(error_str) = error_str {
            match error_str {
                SetError::InvalidExpiredTime => return "Set Error: Invalid expired time".to_string(),
                SetError::KeyOfValueNotSpecified => return "Set Error: Key or value not specified".to_string(),
            }
        } else {
            return "OK".to_string();
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