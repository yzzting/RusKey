use std::str::SplitAsciiWhitespace;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use crate::utils::{get_parts, is_number};

pub fn incrby_float(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // TODO Increment of boundary values is not yet handled
    let (key, value) = get_parts(parts, true);
    if key == "" || value == "" {
        return "ERR wrong number of arguments for command".to_string();
    }
    if !is_number(&value) {
        return "Value is not an float or out of range".to_string();
    }
    let old_value = get(false, parts, &key, db);
    println!("old_value: {}", old_value);
    let value_decimal = match BigDecimal::from_str(&value) {
        Ok(n) => n,
        Err(_) => return "Value is not an float or out of range".to_string(),
    };
    let new_value = if old_value == EMPTY {
        value_decimal
    } else {
        let old_value_decimal = match BigDecimal::from_str(&old_value) {
            Ok(n) => n,
            Err(_) => return "Value is not an float or out of range".to_string(),
        };
        let temp_sum = old_value_decimal + value_decimal;
        // check if temp_sum is out of range
        let min = BigDecimal::from_str("-1.7E308").unwrap();
        let max = BigDecimal::from_str("1.7E308").unwrap();
        if temp_sum < min || temp_sum > max {
            return "Value is not a valid float or out of range".to_string();
        }
        temp_sum
    };
    db.set(key.to_string(), DataType::String(new_value.to_string()));
    new_value.to_string()
}
