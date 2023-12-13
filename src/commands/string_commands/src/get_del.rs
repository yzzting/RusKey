use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use crate::utils::get_parts;

pub fn get_del(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, _) = get_parts(parts, false);
    return if key != "" {
        let value = get(false, parts, &key, db);
        if value == EMPTY {
            return EMPTY.to_string();
        }
        db.delete(&key);
        value
    } else {
        "GetDel Error: Key not specified".to_string()
    }
}
