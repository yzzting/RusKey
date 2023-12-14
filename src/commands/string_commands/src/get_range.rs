use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::{get_parts, slice_from_end};

pub fn get_range(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
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

    let key_value = get(false, parts, &key, db);
    if key_value == EMPTY {
        return "".to_string();
    }
    return slice_from_end(&key_value, start, end);
}
