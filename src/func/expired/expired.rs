use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::Db;
use crate::db::DataType;

const EXPIRED: &str = "expired";

fn get_current_time() -> i32 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp();
    timestamp as i32
}

// splice current time and expired time
fn splice_time(expired: i32) -> String {
    let current_time = get_current_time();
    let expired_time = current_time + expired;
    expired_time.to_string()
}

pub fn handle_expired(key: Option<&str>, value: Option<&str>, db: &mut Db) -> Result<String, &'static str> {
    let key = match key {
        Some(key) => key,
        None => return Err("No such key"),
    };

    if !db.check_expired(key.to_string()) {
        return Err("No such key");
    }

    // match value is number > 0
    let value = match value {
        Some(v) => match v.parse::<i32>() {
            Ok(n) if n > 0 => n,
            _ => return Err("Invalid value"),
        },
        None => return Err("Invalid value"),
    };

    let mut expired_map = match db.get(EXPIRED) {
        Some(DataType::HashMap(expired_map)) => expired_map.clone(),
        None => HashMap::new(),
        _ => return Err("Invalid data type"),
    };

    let expired_time = splice_time(value);
    expired_map.insert(key.to_string(), expired_time);
    db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));

    Ok("OK".to_string())
}