use std::collections::BTreeMap;
use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};

pub fn hmset(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
    let key = match parts.next() {
        Some(key) => key,
        None => return Err("Key not specified"),
    };
    let mut btree_map = BTreeMap::new();
    while let Some(field) = parts.next() {
        let value = match parts.next() {
            Some(value) => value,
            None => return Err("Value not specified"),
        };
        btree_map.insert(field.to_string(), value.to_string());
    }
    db.set(key.to_string(), DataType::ZSet(btree_map));
    Ok("OK".to_string())
}
