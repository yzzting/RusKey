use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use crate::utils::get_parts;

pub fn set_gange(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
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

    let mut old_value = get(false, parts, &key, db);
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
