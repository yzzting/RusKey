use rus_key_db::db::{DataType, Db};

pub fn check_type(key: Option<&str>, db: &mut Db) -> String {
    let key = match key {
        Some(key) => key,
        None => return "none".to_string(),
    };
    if !db.check_expired(key) {
        return "none".to_string();
    }
    match db.get(key) {
        Some(DataType::String(_)) => "string".to_string(),
        // Some(DataType::List(_)) => "list".to_string(),
        // Some(DataType::Set(_)) => "set".to_string(),
        Some(DataType::HashMap(_)) => "hash".to_string(),
        Some(DataType::ZSet(_)) => "zset".to_string(),
        None => "none".to_string(),
    }
}
