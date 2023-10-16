use std::collections::HashMap;

use crate::db::Db;
use crate::db::DataType;

const EXPIRED: &str = "expired";

fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp();
    timestamp
}

// splice current time and expired time
fn splice_time(expired: i64) -> String {
    let current_time = get_current_time();
    let expired_time = current_time + expired;
    expired_time.to_string()
}

fn get_expired_map(db: &mut Db) -> HashMap<String, String> {
    let expired_map = match db.get(EXPIRED) {
        Some(DataType::HashMap(expired_map)) => expired_map.clone(),
        None => HashMap::new(),
        _ => HashMap::new(),
    };
    expired_map
}

pub fn handle_expired(key: Option<&str>, value: Option<&str>, type_str: &str, db: &mut Db) -> Result<String, &'static str> {
    let key = match key {
        Some(key) => key,
        None => return Err("No such key"),
    };

    if !db.check_expired(key.to_string()) {
        return Err("No such key");
    }

    let flag_number = match type_str {
        "" => 0,
        "at" => get_current_time(),
        "p" => 0,
        _ => return Err("Invalid type"),
    };

    let value = match value {
        Some(v) => match v.parse::<i64>() {
            Ok(n) if n > flag_number => n,
            _ => return Err("Invalid value"),
        },
        None => return Err("Invalid value"),
    };

    let mut expired_map = get_expired_map(db);

    let expired_time = if type_str == "" {
        splice_time(value)
    } else {
        value.to_string()
    };

    expired_map.insert(key.to_string(), expired_time);
    db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));

    Ok("OK".to_string())
}

pub fn get_key_expired(key: Option<&str>, db: &mut Db) -> String {
    let key = match key {
        Some(key) => key,
        None => return "No such key".to_string(),
    };

    if !db.check_expired(key.to_string()) {
        return "No such key".to_string();
    }

    let expired_map = get_expired_map(db);

    let current_time = get_current_time();
    let expired_time = match expired_map.get(key) {
        Some(expired_time) => match expired_time.parse::<i64>() {
            Ok(n) if n > current_time => "".to_string(),
            _ => {
                db.delete(key);
                return "nil".to_string();
            }
        },
        None => "".to_string(),
    };
    expired_time
}

pub fn handle_ttl(key: Option<&str>, db: &mut Db) -> Result<String, &'static str> {
    let key = match key {
        Some(key) => key,
        None => return Err("There is no such key, the key is expired, or the data type is incorrect"),
    };

    if !db.check_expired(key.to_string()) {
        return Err("-2");
    }

    let expired_map = get_expired_map(db);

    let current_time = get_current_time();
    let expired_time = match expired_map.get(key) {
        Some(expired_time) => match expired_time.parse::<i64>() {
            Ok(n) if n > current_time => n,
            _ => {
                db.delete(key);
                return Err("-2");
            }
        },
        None => return Err("-2"),
    };
    Ok((expired_time - current_time).to_string())
}