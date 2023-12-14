use std::str::SplitAsciiWhitespace;
use expired_commands::expired::ExpiredCommand;
use rus_key_db::db::Db;
use crate::get::get;
use crate::utils::general_command;
use crate::r#const::{EMPTY, GetExExtraArgs, SetError};
use rus_key_command_lib::get_parts;

pub fn get_ex(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
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
        let value = get(false, parts, &key, db);
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
