use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use crate::utils::get_parts;

pub fn str_len(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, _) = get_parts(parts, false);
    if key.is_empty() {
        return "StrLen Error: Key not specified".to_string();
    }
    let key_value = get(false, parts, &key, db);
    if key_value == EMPTY || key_value.is_empty() {
        return "0".to_string();
    }
    let key_as_str = key_value.as_str();
    return key_as_str.len().to_string();
}
