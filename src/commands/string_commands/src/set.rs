use std::str::SplitAsciiWhitespace;
use expired_commands::expired::{ExpiredCommand, get_key_expired};
use rus_key_db::db::{DataType, Db};
use rus_key_command_lib::get_parts;
use crate::utils::general_command;
use crate::get::get;
use crate::r#const::{ExtraArgs, SetError};

pub fn set(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
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

    if !key.is_empty() {
        // if key exist and extra_args.get is true, return old value
        let old_value = get(false, parts, &key, db);
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
