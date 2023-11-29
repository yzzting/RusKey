use crate::command_factory::Command;
use crate::db::DataType;
use crate::db::Db;
use crate::func::expired::get_key_expired;
use crate::func::expired::ExpiredCommand;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::str::SplitAsciiWhitespace;

enum SetError {
    InvalidExpiredTime,
    KeyOfValueNotSpecified,
}

enum Accumulation {
    Incr = 1,
    Decr = -1,
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

struct GetExExtraArgs {
    ex: Option<i64>,
    px: Option<i64>,
    exat: Option<i64>,
    pxat: Option<i64>,
    persist: Option<bool>,
}

fn general_command(db: &mut Db, command_set: &ExpiredCommand, command_set_str: &str) -> String {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    return result_set.unwrap();
}

fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn get_parts(parts: &mut SplitAsciiWhitespace, get_value: bool) -> (String, String) {
    let key = match parts.next() {
        Some(key) => key.to_string(),
        None => "".to_string(),
    };
    let value = if get_value {
        match parts.next() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        }
    } else {
        "".to_string()
    };
    (key, value)
}

pub struct StringCommand {
    command: String,
}

impl StringCommand {
    pub fn new(command: String) -> StringCommand {
        StringCommand { command }
    }

    fn append(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, value) = get_parts(parts, true);
        if key != "" && value != "" {
            let mut old_value = StringCommand::get(self, &key, db);
            if old_value == "nil" {
                old_value = "".to_string();
            }
            old_value.push_str(&value);
            let len = old_value.len();
            db.set(key.to_string(), DataType::String(old_value));
            return format!("{}", len);
        } else {
            return "Append Error: Key or value not specified".to_string();
        }
    }

    fn get_del(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, _) = get_parts(parts, false);
        if key != "" {
            let value = StringCommand::get(self, &key, db);
            if value == "nil" {
                return "nil".to_string();
            }
            db.delete(&key);
            return value;
        } else {
            return "GetDel Error: Key not specified".to_string();
        }
    }

    fn get_ex(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, _) = get_parts(parts, false);
        // ex px command
        let expired_command = ExpiredCommand::new("expired".to_string());
        // exat command
        let expired_at_command = ExpiredCommand::new("expireat".to_string());
        // pxat command
        let pexpired_at_command = ExpiredCommand::new("pexpireat".to_string());
        // persist command
        let persist_command = ExpiredCommand::new("persist".to_string());
        // extra object
        let mut extra_args = GetExExtraArgs {
            ex: None,
            px: None,
            exat: None,
            pxat: None,
            persist: None,
        };
        // ex px exat and pxat cannot exist simultaneously set a counter to count them
        let mut expired_count = 0;

        let mut error_str: Option<SetError> = None;
        while let Some(arg) = parts.next() {
            let lower_arg = arg.to_lowercase();
            println!("lower_arg: {}", lower_arg);
            match lower_arg.as_str() {
                "ex" => {
                    expired_count += 1;
                    if let Some(seconds_str) = parts.next() {
                        let seconds = seconds_str.parse::<i64>().unwrap();
                        extra_args.ex = Some(seconds);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                }
                "px" => {
                    expired_count += 1;
                    if let Some(milliseconds_str) = parts.next() {
                        let milliseconds = milliseconds_str.parse::<i64>().unwrap();
                        extra_args.px = Some((milliseconds + 999) / 1000);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                }
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
                }
                "persist" => {
                    expired_count += 1;
                    extra_args.persist = Some(true);
                }
                _ => {}
            }

            // ex/px and exat/pxat cannot exist simultaneously
            if expired_count > 1 {
                return "Set Error: Invalid expired time in set".to_string();
            }
        }

        if key != "" {
            let value = StringCommand::get(self, &key, db);
            if value == "nil" {
                return "nil".to_string();
            }

            // handle extra arg
            if extra_args.ex.is_some() {
                let result = general_command(
                    db,
                    &expired_command,
                    &format!("{} {}", key, extra_args.ex.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.px.is_some() {
                let result = general_command(
                    db,
                    &expired_command,
                    &format!("{} {}", key, extra_args.px.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.exat.is_some() {
                let result = general_command(
                    db,
                    &expired_at_command,
                    &format!("{} {}", key, extra_args.exat.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.pxat.is_some() {
                let result = general_command(
                    db,
                    &pexpired_at_command,
                    &format!("{} {}", key, extra_args.pxat.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.persist.is_some() {
                let result = general_command(db, &persist_command, &key);
                if result != "1" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }

            if let Some(error_str) = error_str {
                match error_str {
                    SetError::InvalidExpiredTime => {
                        return "Set Error: Invalid expired time".to_string()
                    }
                    SetError::KeyOfValueNotSpecified => {
                        return "Set Error: Key or value not specified".to_string()
                    }
                }
            } else {
                return value;
            }
        } else {
            return "Key not specified".to_string();
        }
    }

    fn get_set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, value) = get_parts(parts, true);

        if key != "" && value != "" {
            let old_value = StringCommand::get(self, &key, db);
            db.set(key.to_string(), DataType::String(value.to_string()));
            // if old_value is nil, return nil else return old_value
            if old_value == "nil" {
                return "nil".to_string();
            } else {
                return old_value;
            }
        } else {
            return "GetSet Error: Key or value not specified".to_string();
        }
    }

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, mut value) = get_parts(parts, true);
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
        if value.starts_with('"') && !value.ends_with('"') {
            while let Some(part) = parts.next() {
                value.push_str(" ");
                value.push_str(part);
                if part.ends_with('"') {
                    break;
                }
            }
        }
        value = value.trim_matches('"').to_string();
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
                }
                "px" => {
                    expired_count += 1;
                    if let Some(milliseconds_str) = parts.next() {
                        let milliseconds = milliseconds_str.parse::<i64>().unwrap();
                        extra_args.px = Some((milliseconds + 999) / 1000);
                    } else {
                        error_str = Some(SetError::InvalidExpiredTime);
                    }
                }
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
                }
                "nx" => {
                    extra_args.nx = Some(true);
                }
                "xx" => {
                    extra_args.xx = Some(true);
                }
                "keepttl" => {
                    extra_args.keepttl = Some(true);
                }
                "get" => {
                    extra_args.get = Some(true);
                }
                _ => {}
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
        if db.check_expired(&key) && extra_args.nx.is_some() {
            return "Set Error: Key already exists".to_string();
        }

        // xx must exist
        if !db.check_expired(&key) && extra_args.xx.is_some() {
            return "Set Error: Key does not exist".to_string();
        }

        if key != "" {
            // if key exist and extra_args.get is true, return old value
            let old_value = StringCommand::get(self, &key, db);
            if !old_value.is_empty() && extra_args.get.is_some() {
                return_value = old_value;
            }
            db.set(key.to_string(), DataType::String(value.to_string()));
            // hangle extra arg
            if extra_args.ex.is_some() {
                let result = general_command(
                    db,
                    &expired_command,
                    &format!("{} {}", key, extra_args.ex.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.px.is_some() {
                let result = general_command(
                    db,
                    &expired_command,
                    &format!("{} {}", key, extra_args.px.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.exat.is_some() {
                let result = general_command(
                    db,
                    &expired_at_command,
                    &format!("{} {}", key, extra_args.exat.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            if extra_args.pxat.is_some() {
                let result = general_command(
                    db,
                    &pexpired_at_command,
                    &format!("{} {}", key, extra_args.pxat.unwrap_or(0)),
                );
                if result != "OK" {
                    error_str = Some(SetError::InvalidExpiredTime);
                }
            }
            let key_expired = get_key_expired(Some(&key), db);
            // if key not expired and not expired time arg, set expired time to nil
            if !key_expired.is_empty() && expired_count == 0 && extra_args.keepttl.is_none() {
                general_command(db, &persist_command, &key);
            }
        } else {
            error_str = Some(SetError::KeyOfValueNotSpecified);
        }

        if let Some(error_str) = error_str {
            match error_str {
                SetError::InvalidExpiredTime => {
                    return "Set Error: Invalid expired time".to_string()
                }
                SetError::KeyOfValueNotSpecified => {
                    return "Set Error: Key or value not specified".to_string()
                }
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
            _ => "There is no such key, the key is expired, or the data type is incorrect"
                .to_string(),
        }
    }

    fn handle_accumulation(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
        accumulation: Accumulation,
        is_by: bool,
    ) -> String {
        // match accumulation incr or decr
        let accumulation_str = match accumulation {
            Accumulation::Incr => 1,
            Accumulation::Decr => -1,
        };
        let (key, _) = get_parts(parts, false);
        if key == "" {
            return "ERR wrong number of arguments for command".to_string();
        }
        // is_by true get num value
        let num: Option<i128> = if is_by {
            match parts.next() {
                Some(n) => n.parse::<i128>().ok(),
                None => None,
            }
        } else {
            Some(1)
        };

        let num_value = match num {
            Some(n) => n,
            None => return "ERR wrong number of arguments for command".to_string(),
        };

        let old_value = StringCommand::get(self, &key, db);

        // old_value is nil
        let new_value = if old_value == "nil" {
            // accumulation_str == Incr is 1 or Decr is -1
            accumulation_str
        } else {
            // check if old_value is not an integer
            if !is_integer(&old_value) {
                return "Value is not an integer or out of range".to_string();
            }
            // old_value is an integer
            match old_value.parse::<i128>() {
                Ok(n) => {
                    println!("n: {}, num_value: {}", n, num_value);
                    // accumulation_str == Incr is n + num_value or Decr is n - num_value
                    let new_value = n + accumulation_str * num_value;
                    // (n + num_value) as i64
                    if new_value < i64::MIN as i128 || new_value > i64::MAX as i128 {
                        return "Value is not an integer or out of range".to_string();
                    }
                    new_value
                }
                Err(_) => return "Value is not an integer or out of range".to_string(),
            }
        };

        db.set(key.to_string(), DataType::String(new_value.to_string()));
        new_value.to_string()
    }

    fn incrby_float(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        // TODO Increment of boundary values is not yet handled
        let (key, value) = get_parts(parts, true);
        if key == "" || value == "" {
            return "ERR wrong number of arguments for command".to_string();
        }
        if !is_number(&value) {
            return "Value is not an float or out of range".to_string();
        }
        let old_value = StringCommand::get(self, &key, db);
        println!("old_value: {}", old_value);
        let value_decimal = match BigDecimal::from_str(&value) {
            Ok(n) => n,
            Err(_) => return "Value is not an float or out of range".to_string(),
        };
        let new_value = if old_value == "nil" {
            value_decimal
        } else {
            let old_value_decimal = match BigDecimal::from_str(&old_value) {
                Ok(n) => n,
                Err(_) => return "Value is not an float or out of range".to_string(),
            };
            let temp_sum = old_value_decimal + value_decimal;
            // check if temp_sum is out of range
            let min = BigDecimal::from_str("-1.7E308").unwrap();
            let max = BigDecimal::from_str("1.7E308").unwrap();
            if temp_sum < min || temp_sum > max {
                return "Value is not a valid float or out of range".to_string();
            }
            temp_sum
        };
        db.set(key.to_string(), DataType::String(new_value.to_string()));
        new_value.to_string()
    }

    fn get_range(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
        let (key, _) = get_parts(parts, false);
        if key == "" {
            return "GetRange Error: Key not specified".to_string();
        }
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

        let key_value = self.get(&key, db);
        if key_value == "nil" {
            return "".to_string();
        }
        return StringCommand::slice_from_end(&key_value, start, end);
    }

    fn slice_from_end(str: &str, start: isize, end: isize) -> String {
        if start > end {
            return "".to_string();
        }
        let char_vec: Vec<char> = str.chars().collect();
        let char_vec_len = char_vec.len() as isize;
        let (start, end) = if start < 0 || end < 0 {
            (
                (char_vec_len + start).max(0) as usize,
                (char_vec_len + end + 1).max(0) as usize,
            )
        } else {
            (start as usize, (end + 1).min(char_vec_len) as usize)
        };
        char_vec[start..end].iter().collect::<String>()
    }
}

impl Command for StringCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "append" => Ok(self.append(parts, db)),
            "decr" => Ok(self.handle_accumulation(parts, db, Accumulation::Decr, false)),
            "decrby" => Ok(self.handle_accumulation(parts, db, Accumulation::Decr, true)),
            "get" => Ok(self.get(parts.next().unwrap(), db)),
            "getdel" => Ok(self.get_del(parts, db)),
            "getex" => Ok(self.get_ex(parts, db)),
            "incr" => Ok(self.handle_accumulation(parts, db, Accumulation::Incr, false)),
            "incrby" => Ok(self.handle_accumulation(parts, db, Accumulation::Incr, true)),
            "incrbyfloat" => Ok(self.incrby_float(parts, db)),
            "getrange" => Ok(self.get_range(parts, db)),
            "getset" => Ok(self.get_set(parts, db)),
            "set" => Ok(self.set(parts, db)),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}
