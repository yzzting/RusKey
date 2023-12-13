use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use crate::utils::get_parts;

pub fn append(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, value) = get_parts(parts, true);
    return if !key.is_empty() && !value.is_empty() {
        let mut old_value = get(false, parts, &key, db);
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
