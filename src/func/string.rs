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

struct ExtraArgs {
    ex: Option<i64>,
    px: Option<i64>,
    exat: Option<i64>,
    pxat: Option<i64>,
    nx: Option<bool>,
    xx: Option<bool>,
    keepttl: Option<bool>,
    get: Option<bool>,
}

fn general_command(db: &mut Db, command_set: &ExpiredCommand, command_set_str: &str) -> String {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    return result_set.unwrap();
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

    fn append(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let key = parts.next();
        let value = parts.next();
        if let (Some(key), Some(value)) = (key, value) {
            let mut old_value = StringCommand::get(self, key, db);
            if old_value == "nil" {
                old_value = "".to_string();
            }
            old_value.push_str(value);
            let len = old_value.len();
            db.set(key.to_string(), DataType::String(old_value));
            return format!("{}", len);
        } else {
            return "Append Error: Key or value not specified".to_string();
        }
    }

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        // ex px command
        let expired_command = ExpiredCommand::new("expired".to_string());
        // exat command
        let expired_at_command = ExpiredCommand::new("expireat".to_string());
        // pxat command
        let pexpired_at_command = ExpiredCommand::new("pexpireat".to_string());
        // persist command
        let persist_command = ExpiredCommand::new("persist".to_string());
        // extra object
        let mut extra_args = ExtraArgs {
            ex: None,
            px: None,
            exat: None,
            pxat: None,
            nx: None,
            xx: None,
            keepttl: None,
            get: None,
        };
        // return value OK or old value
        let mut return_value = "OK".to_string();
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
        let mut error_str: Option<SetError> = None;

        // After parsing value, parse all remaining args
        // ex px exat and pxat cannot exist simultaneously set a counter to count them
        let mut expired_count = 0;
        // if nx or xx is specified, the key must not exist or must exist
        while let Some(arg) = parts.next() {
            let lower_arg = arg.to_lowercase();
            match lower_arg.as_str() {
                "ex" => {
                    expired_count += 1;
                    if let Some(seconds_str) = parts.next() {
                        let seconds = seconds_str.parse::<i64>().unwrap();
                        extra_args.ex = Some(seconds);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                },
                "px" => {
                    expired_count += 1;
                    if let Some(milliseconds_str) = parts.next() {
                        let milliseconds = milliseconds_str.parse::<i64>().unwrap();
                        extra_args.px = Some((milliseconds + 999) / 1000);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                },
                "exat" | "pxat" => {
                    expired_count += 1;
                    if let Some(timestamp) = parts.next() {
                        let timestamp = timestamp.parse::<i64>().unwrap();
                        if lower_arg == "exat" {
                            extra_args.exat = Some(timestamp);
                        } else {
                            extra_args.pxat = Some(timestamp);
                        }
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                },
                "nx" => {
                    extra_args.nx = Some(true);
                },
                "xx" => {
                    extra_args.xx = Some(true);
                },
                "keepttl" => {
                    extra_args.keepttl = Some(true);
                },
                "get" => {
                    extra_args.get = Some(true);
                },
                _ => {},
            }
        }

        // ex/px and exat/pxat cannot exist simultaneously
        if expired_count > 1 {
            return "Set Error: Invalid expired time in set".to_string();
        }

        // nx and xx cannot exist simultaneously
        if extra_args.nx.is_some() && extra_args.xx.is_some() {
            return "Set Error: nx and xx cannot exist simultaneously".to_string();
        }

        // nx must not exist
        if db.check_expired(key.unwrap()) && extra_args.nx.is_some() {
            return "Set Error: Key already exists".to_string();
        }

        // xx must exist
        if !db.check_expired(key.unwrap()) && extra_args.xx.is_some() {
            return "Set Error: Key does not exist".to_string();
        }

        if let (Some(key), Some(value)) = (key, value) {
            // if key exist and extra_args.get is true, return old value
            let old_value = StringCommand::get(self, key, db);
            if !old_value.is_empty() && extra_args.get.is_some() {
                return_value = old_value;
            }
            db.set(key.to_string(), DataType::String(value.to_string()));
            // hangle extra arg
            if extra_args.ex.is_some() {
                let result = general_command(db, &expired_command, &format!("{} {}", key, extra_args.ex.unwrap_or(0)));
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.px.is_some() {
                let result = general_command(db, &expired_command, &format!("{} {}", key, extra_args.px.unwrap_or(0)));
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.exat.is_some() {
                let result = general_command(db, &expired_at_command, &format!("{} {}", key, extra_args.exat.unwrap_or(0)));
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.pxat.is_some() {
                let result = general_command(db, &pexpired_at_command, &format!("{} {}", key, extra_args.pxat.unwrap_or(0)));
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            let key_expired = get_key_expired(Some(key), db);
            // if key not expired and not expired time arg, set expired time to nil
            if !key_expired.is_empty() && expired_count == 0 && extra_args.keepttl.is_none() {
                general_command(db, &persist_command, key);
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
            return return_value.to_string();
        }
    }

    fn get(&self, key: &str, db: &mut Db) -> String {
        // check expired
        if !db.check_expired(key) {
            return "nil".to_string();
        }

        let expired = get_key_expired(Some(key), db);

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
        if key_value == "nil" {
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
            "append" => Ok(self.append(parts, db)),
            "set" => Ok(self.set(parts, db)),
            "get" => Ok(self.get(parts.next().unwrap(), db)),
            "getrange" => Ok(self.get_range(parts, db)),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}