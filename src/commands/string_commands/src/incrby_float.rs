use std::str::SplitAsciiWhitespace;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::{EMPTY, MIN_VALUE, MAX_VALUE};
use rus_key_command_lib::{get_parts, is_number};

const INCRBY_FLOAT_ERROR: &str = "The value is not a valid float";

/// Increments the value of a key in the database by a specified float value.
///
/// This function is designed to increment the value of a key in a database by a specified float value. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database where the value is incremented.
///
/// The function first retrieves the key and the increment value from the `parts` iterator. If no key or increment value is provided, or if the increment value is not a number, it returns an error message.
///
/// Next, it retrieves the old value associated with the key from the database.
///
/// If the old value is `EMPTY`, the new value is set to the increment value. Otherwise, the function attempts to parse the old value as a `BigDecimal`. If the parsing fails, it returns an error message.
///
/// The function then adds the increment value to the old value to calculate the new value. If the new value is out of range, it returns an error message.
///
/// Finally, it sets the new value in the database and returns the new value as a string.
pub fn incrby_float(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve the key and the increment value from the `parts` iterator
    let (key, value) = get_parts(parts, true);
    if key.is_empty() || value.is_empty() {
        return "ERR wrong number of arguments for command".to_string();
    }
    // If the increment value is not a number, return an error message
    if !is_number(&value) {
        return INCRBY_FLOAT_ERROR.to_string();
    }
    // Retrieve the old value associated with the key from the database
    let old_value = get(false, parts, &key, db);
    // Attempt to parse the increment value as a `BigDecimal`
    let value_decimal = match BigDecimal::from_str(&value) {
        Ok(n) => n,
        Err(_) => return INCRBY_FLOAT_ERROR.to_string(),
    };
    // If the old value is `EMPTY`, set the new value to the increment value
    let new_value = if old_value == EMPTY {
        value_decimal
    } else {
        // Attempt to parse the old value as a `BigDecimal`
        let old_value_decimal = match BigDecimal::from_str(&old_value) {
            Ok(n) => n,
            Err(_) => return INCRBY_FLOAT_ERROR.to_string(),
        };
        // Add the increment value to the old value to calculate the new value
        let temp_sum = old_value_decimal + value_decimal;
        // If the new value is out of range, return an error message
        if temp_sum < *MIN_VALUE || temp_sum > *MAX_VALUE {
            return INCRBY_FLOAT_ERROR.to_string();
        }
        temp_sum
    };
    // Set the new value in the database
    db.set(key.to_string(), DataType::String(new_value.to_string()));
    // Return the new value as a string
    new_value.to_string()
}
