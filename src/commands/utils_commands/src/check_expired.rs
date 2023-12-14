use rus_key_db::db::Db;

pub fn check_expired(key: Option<&str>, db: &mut Db) -> String {
    let key = match key {
        Some(key) => key,
        None => return "0".to_string(),
    };
    if db.check_expired(key) {
        return "1".to_string();
    }
    "0".to_string()
}
