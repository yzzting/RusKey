use crate::db::Db;

pub fn rename(old_name: Option<&str>, new_name: Option<&str>, db: &mut Db) -> Result<String, &'static str> {
    let old_name = match old_name {
        Some(old_name) => old_name,
        None => return Err("No such key"),
    };

    let new_name = match new_name {
        Some(new_name) => new_name,
        None => return Err("No such key"),
    };

    if db.check_expired(new_name.to_string()) {
        return Err("new name is exists");
    }

    let value = match db.get(old_name) {
        Some(value) => value.clone(),
        None => return Err("No such key"),
    };

    db.set(new_name.to_string(), value);

    if !db.delete(old_name) {
        return Err("Failed to delete old key");
    }

    Ok("OK".to_string())
}