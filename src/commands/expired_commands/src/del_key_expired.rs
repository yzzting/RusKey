use rus_key_db::db::{DataType, Db};
use crate::r#const::EXPIRED;
use crate::utils::get_expired_map;

pub fn del_key_expired(key: Option<&str>, db: &mut Db) -> String {
    let key = match key {
        Some(key) => key,
        None => return "0".to_string(),
    };

    let mut expired_map = get_expired_map(db);

    if !expired_map.contains_key(key) {
        return "0".to_string();
    }

    expired_map.remove(key);
    db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));

    "1".to_string()
}
