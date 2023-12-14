use rus_key_db::db::{DataType, Db};
use crate::r#const::EXPIRED;
use crate::utils::{get_current_time, get_expired_map, splice_time};

pub fn handle_expired(
    key: Option<&str>,
    value: Option<&str>,
    type_str: &str,
    db: &mut Db,
) -> Result<String, &'static str> {
    let key = match key {
        Some(key) => key,
        None => return Err("No such key"),
    };

    if !db.check_expired(&key) {
        return Err("No such key");
    }

    let (flag_number, multiplier) = match type_str {
        "" => (0, 1),
        "at" => (get_current_time() / 1000, 1000),
        "p" => (get_current_time(), 1),
        _ => return Err("Invalid type"),
    };

    let value = match value {
        Some(v) => match v.parse::<i64>() {
            Ok(n) if n > flag_number => n * multiplier,
            _ => return Err("Invalid value"),
        },
        None => return Err("Invalid value"),
    };

    let mut expired_map = get_expired_map(db);

    let expired_time = if type_str == "" {
        splice_time(value * 1000)
    } else {
        value
    };

    expired_map.insert(key.to_string(), expired_time.to_string());
    db.set(EXPIRED.to_string(), DataType::HashMap(expired_map));

    Ok("OK".to_string())
}
