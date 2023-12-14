use rus_key_db::db::Db;
use crate::utils::{get_current_time, get_expired_map};

pub fn handle_ttl(key: Option<&str>, type_str: &str, db: &mut Db) -> i64 {
    let key = match key {
        Some(key) => key,
        None => return -2,
    };

    if !db.check_expired(key) {
        return -2;
    }

    let expired_map = get_expired_map(db);

    let current_time = get_current_time();
    let expired_time = match expired_map.get(key) {
        Some(expired_time) => match expired_time.parse::<i64>() {
            Ok(n) if n > current_time => n,
            _ => {
                db.delete(key);
                return -2;
            }
        },
        None => return -1,
    };
    let multiplier = match type_str {
        "" => 1,
        "p" => 1000,
        _ => 1,
    };
    ((expired_time - current_time) / 1000) * multiplier
}
