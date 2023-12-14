use std::collections::HashMap;
use rus_key_db::db::{DataType, Db};

const EXPIRED: &str = "expired";

pub fn rename(
    old_name: Option<&str>,
    new_name: Option<&str>,
    type_str: &str,
    db: &mut Db,
) -> Result<String, &'static str> {
    let old_name = match old_name {
        Some(old_name) => old_name,
        None => return Err("No such key"),
    };

    let new_name = match new_name {
        Some(new_name) => new_name,
        None => return Err("No such key"),
    };

    if !db.check_expired(old_name) {
        return Err("No such key");
    }

    if db.check_expired(new_name) && type_str == "nx" {
        return Err("New name is exists");
    }

    let mut expired_map = match db.get(EXPIRED) {
        Some(DataType::HashMap(expired_map)) => expired_map.clone(),
        None => HashMap::new(),
        _ => HashMap::new(),
    };

    // if expired_map contains old_name, insert new_name
    if expired_map.contains_key(old_name) {
        let old_expired = match expired_map.get(old_name) {
            Some(old_expired) => old_expired,
            None => return Err("No such key"),
        };

        expired_map.insert(new_name.to_string(), old_expired.to_string());
        db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));
    }

    let value = match db.get(old_name) {
        Some(value) => value.clone(),
        None => return Err("No such key"),
    };

    db.set(new_name.to_string(), value);

    if !db.delete(old_name) {
        return Err("Failed to delete old key");
    }

    Ok("1".to_string())
}
