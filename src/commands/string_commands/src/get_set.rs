use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

pub fn get_set(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, value) = get_parts(parts, true);

    return if !key.is_empty() && !value.is_empty() {
        let old_value = get(false, parts, &key, db);
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
