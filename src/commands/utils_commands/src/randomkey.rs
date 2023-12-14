use rus_key_db::db::Db;

pub fn randomkey(db: &mut Db) -> String {
    match db.randomkey() {
        Some(key) => key,
        None => "nil".to_string(),
    }
}
