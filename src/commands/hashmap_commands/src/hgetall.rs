use std::str::SplitAsciiWhitespace;
use expired_commands::expired::get_key_expired;
use rus_key_db::db::{DataType, Db};

pub fn hgetall(
    parts: &mut SplitAsciiWhitespace,
    db: &mut Db,
) -> Result<String, &'static str> {
    let key = match parts.next() {
        Some(key) => key,
        None => return Err("Key not specified"),
    };
    // check expired
    let expired = get_key_expired(Some(key), db);
    if !expired.is_empty() && expired != "nil" {
        return Err("There is no such key, the key is expired, or the data type is incorrect");
    }
    if expired == "nil" {
        return Err("nil");
    }
    match db.get(key) {
        Some(DataType::ZSet(btree_map)) => {
            let mut result = String::new();
            for (field, value) in btree_map {
                result.push_str(&format!("{}: {} ", field, value));
            }
            Ok(result.trim().to_string())
        }
        _ => Err("There is no such key, the key is expired, or the data type is incorrect"),
    }
}
