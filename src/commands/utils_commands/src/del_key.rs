use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;

pub fn del_key(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let mut count = 0;
    while let Some(key) = parts.next() {
        if db.delete(key) {
            count += 1;
        }
    }
    count.to_string()
}
