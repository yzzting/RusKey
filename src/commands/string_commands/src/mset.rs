use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use rus_key_command_lib::get_value;

pub fn mset(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
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
