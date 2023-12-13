use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::utils::get_value;

pub fn mget(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let mut key_vec: Vec<String> = Vec::new();
    while let Some(key) = parts.next() {
        let key = get_value(key.to_string(), parts);
        key_vec.push(key.to_string());
    }
    let mut value_vec: Vec<String> = Vec::new();
    for key in key_vec {
        let value = get(false, parts, &key, db);
        value_vec.push(value);
    }
    value_vec.join(" ")
}
