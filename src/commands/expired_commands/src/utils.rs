use std::collections::HashMap;
use rus_key_db::db::{DataType, Db};
use crate::r#const::EXPIRED;

pub fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp_millis();
    timestamp
}

// splice current time and expired time
pub fn splice_time(expired: i64) -> i64 {
    let current_time = get_current_time();
    let expired_time = current_time + expired;
    expired_time
}

pub fn get_expired_map(db: &mut Db) -> HashMap<String, String> {
    let expired_map = match db.get(EXPIRED) {
        Some(DataType::HashMap(expired_map)) => expired_map.clone(),
        None => HashMap::new(),
        _ => HashMap::new(),
    };
    expired_map
}
