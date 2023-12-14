use std::str::SplitAsciiWhitespace;
use expired_commands::expired::get_key_expired;
use rus_key_db::db::{DataType, Db};
use rus_key_command_lib::get_parts;
use crate::r#const::EMPTY;

pub fn get(
    is_parts: bool,
    parts: &mut SplitAsciiWhitespace,
    key: &str,
    db: &mut Db,
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
