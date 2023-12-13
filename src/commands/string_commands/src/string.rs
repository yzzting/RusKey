use rus_key_trait::command_trait::Command;
use rus_key_trait::db_trait::{Db, DataType};
use expired_commands::expired::{ExpiredCommand, get_key_expired};
use crate::utils::{is_integer, general_command, get_value, get_parts, is_number, slice_from_end};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::str::SplitAsciiWhitespace;

const EMPTY: &str = "nil";

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

pub struct StringCommand {
    command: String,
}

impl StringCommand {
    pub fn new(command: String) -> StringCommand {
        StringCommand { command }
    }

    fn append(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, value) = get_parts(parts, true);
        return if !key.is_empty() && !value.is_empty() {
            let mut old_value = self.get(false, parts, &key, db);
            if old_value == EMPTY {
                old_value = "".to_string();
            }
            old_value.push_str(&value);
            let len = old_value.len();
            db.set(key.to_string(), DataType::String(old_value));
            format!("{}", len)
        } else {
            "Append Error: Key or value not specified".to_string()
        }
    }

    fn get_del(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, _) = get_parts(parts, false);
        return if key != "" {
            let value = self.get(false, parts, &key, db);
            if value == EMPTY {
                return EMPTY.to_string();
            }
            db.delete(&key);
            value
        } else {
            "GetDel Error: Key not specified".to_string()
        }
    }

    fn get_ex(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
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

        return if !key.is_empty() {
            let value = self.get(false, parts, &key, db);
            if value == EMPTY {
                return EMPTY.to_string();
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
                        "Set Error: Invalid expired time".to_string()
                    }
                    SetError::KeyOfValueNotSpecified => {
                        "Set Error: Key or value not specified".to_string()
                    }
                }
            } else {
                value
            }
        } else {
            "Key not specified".to_string()
        }
    }

    fn get_set(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, value) = get_parts(parts, true);

        return if key != "" && value != "" {
            let old_value = self.get(false, parts, &key, db);
            db.set(key.to_string(), DataType::String(value.to_string()));
            // if old_value is nil, return nil else return old_value
            if old_value == EMPTY {
                EMPTY.to_string()
            } else {
                old_value
            }
        } else {
            "GetSet Error: Key or value not specified".to_string()
        }
    }

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, value) = get_parts(parts, true);
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
            let old_value = self.get(false, parts, &key, db);
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

        return if let Some(error_str) = error_str {
            match error_str {
                SetError::InvalidExpiredTime => {
                    "Set Error: Invalid expired time".to_string()
                }
                SetError::KeyOfValueNotSpecified => {
                    "Set Error: Key or value not specified".to_string()
                }
            }
        } else {
            return_value.to_string()
        }
    }

    fn get(
        &self,
        is_parts: bool,
        parts: &mut SplitAsciiWhitespace,
        key: &str,
        db: &mut dyn Db,
    ) -> String {
        // if is_parts is true, get key from get_parts, else get key from key
        let key = if is_parts {
            let (key, _) = get_parts(parts, false);
            key
        } else {
            key.to_string()
        };
        // check expired
        if !db.check_expired(&key) {
            return EMPTY.to_string();
        }
        let expired = get_key_expired(Some(&key), db);

        if expired == EMPTY {
            return EMPTY.to_string();
        }
        match db.get(&key) {
            Some(DataType::String(value)) => value.clone(),
            _ => "There is no such key, the key is expired, or the data type is incorrect"
                .to_string(),
        }
    }

    fn handle_accumulation(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut dyn Db,
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

        let old_value = self.get(false, parts, &key, db);

        // old_value is nil
        let new_value = if old_value == EMPTY {
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

    fn incrby_float(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        // TODO Increment of boundary values is not yet handled
        let (key, value) = get_parts(parts, true);
        if key == "" || value == "" {
            return "ERR wrong number of arguments for command".to_string();
        }
        if !is_number(&value) {
            return "Value is not an float or out of range".to_string();
        }
        let old_value = self.get(false, parts, &key, db);
        println!("old_value: {}", old_value);
        let value_decimal = match BigDecimal::from_str(&value) {
            Ok(n) => n,
            Err(_) => return "Value is not an float or out of range".to_string(),
        };
        let new_value = if old_value == EMPTY {
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

    fn get_range(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
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

        let key_value = self.get(false, parts, &key, db);
        if key_value == EMPTY {
            return "".to_string();
        }
        return slice_from_end(&key_value, start, end);
    }

    fn str_len(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, _) = get_parts(parts, false);
        if key == "" {
            return "StrLen Error: Key not specified".to_string();
        }
        let key_value = self.get(false, parts, &key, db);
        if key_value == EMPTY || key_value == "" {
            return "0".to_string();
        }
        let key_as_str = key_value.as_str();
        return key_as_str.len().to_string();
    }

    fn set_gange(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let (key, str_num) = get_parts(parts, true);
        let new_value = match parts.next() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };

        let num = match str_num.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return "ERR wrong number of arguments for command".to_string(),
        };

        if new_value.is_empty() {
            return "ERR wrong number of arguments for command".to_string();
        }

        let mut old_value = self.get(false, parts, &key, db);
        // old value length nil is 0
        let old_value_len = if old_value == EMPTY {
            old_value.clear();
            0
        } else {
            old_value.len()
        };
        // calculate num and new_value length
        let required_capacity = num + new_value.len();
        let mut value_with_capacity = String::with_capacity(required_capacity);

        if old_value_len < num {
            let padding = " ".repeat(num - old_value_len);
            value_with_capacity.push_str(&old_value);
            value_with_capacity.push_str(&padding);
        } else {
            let truncate_value = &old_value[..num];
            value_with_capacity.push_str(truncate_value);
        }

        value_with_capacity.push_str(&new_value);

        let len = value_with_capacity.len();

        db.set(key.to_string(), DataType::String(value_with_capacity));

        len.to_string()
    }

    fn mset(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let mut key_value_vec: Vec<(String, String)> = Vec::new();

        while let Some(key) = parts.next() {
            let key = get_value(key.to_string(), parts);
            let value = match parts.next() {
                Some(value) => value.to_string(),
                None => return "wrong number of arguments for 'mset' command".to_string(),
            };
            let value = get_value(value, parts);
            key_value_vec.push((key.to_string(), value));
        }
        for (key, value) in key_value_vec {
            db.set(key.to_string(), DataType::String(value.to_string()));
        }
        "OK".to_string()
    }

    fn mget(&self, parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> String {
        let mut key_vec: Vec<String> = Vec::new();
        while let Some(key) = parts.next() {
            let key = get_value(key.to_string(), parts);
            key_vec.push(key.to_string());
        }
        let mut value_vec: Vec<String> = Vec::new();
        for key in key_vec {
            let value = self.get(false, parts, &key, db);
            value_vec.push(value);
        }
        value_vec.join(" ")
    }
}

impl Command for StringCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut dyn Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "append" => Ok(self.append(parts, db)),
            "decr" => Ok(self.handle_accumulation(parts, db, Accumulation::Decr, false)),
            "decrby" => Ok(self.handle_accumulation(parts, db, Accumulation::Decr, true)),
            "get" => Ok(self.get(true, parts, "", db)),
            "getdel" => Ok(self.get_del(parts, db)),
            "getex" => Ok(self.get_ex(parts, db)),
            "incr" => Ok(self.handle_accumulation(parts, db, Accumulation::Incr, false)),
            "incrby" => Ok(self.handle_accumulation(parts, db, Accumulation::Incr, true)),
            "incrbyfloat" => Ok(self.incrby_float(parts, db)),
            "getrange" => Ok(self.get_range(parts, db)),
            "getset" => Ok(self.get_set(parts, db)),
            "set" => Ok(self.set(parts, db)),
            "mset" => Ok(self.mset(parts, db)),
            "mget" => Ok(self.mget(parts, db)),
            "setrange" => Ok(self.set_gange(parts, db)),
            "strlen" => Ok(self.str_len(parts, db)),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}
