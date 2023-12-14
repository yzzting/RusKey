use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

const SET_RANGE_ERROR: &str = "ERR wrong number of arguments for command";

/// Sets a range of characters in the value of a key in the database.
///
/// This function is designed to set a range of characters in the value of a key in a database. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database where the value is set.
///
/// The function first retrieves the key, the start index of the range, and the new value from the `parts` iterator. If any of these are not provided, or if the start index cannot be parsed as a `usize`, it returns an error message.
///
/// Next, it retrieves the old value associated with the key from the database. If the old value is `EMPTY`, it clears the old value and sets its length to 0.
///
/// The function then calculates the required capacity for the new value. If the length of the old value is less than the start index, it pads the old value with spaces until it reaches the start index. Otherwise, it truncates the old value at the start index.
///
/// The function then appends the new value to the old value and calculates the length of the resulting string.
///
/// Finally, it sets the new value in the database and returns the length of the new value as a string.
pub fn set_range(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve the key, the start index of the range, and the new value from the `parts` iterator
    let (key, str_num) = get_parts(parts, true);
    let new_value = match parts.next() {
        Some(value) => value.to_string(),
        None => "".to_string(),
    };

    let num = match str_num.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return SET_RANGE_ERROR.to_string(),
    };

    if new_value.is_empty() {
        return SET_RANGE_ERROR.to_string();
    }

    // Retrieve the old value associated with the key from the database
    let mut old_value = get(false, parts, &key, db);
    // If the old value is `EMPTY`, clear the old value and set its length to 0
    let old_value_len = if old_value == EMPTY {
        old_value.clear();
        0
    } else {
        old_value.len()
    };
    // Calculate the required capacity for the new value
    let required_capacity = num + new_value.len();
    let mut value_with_capacity = String::with_capacity(required_capacity);

    // If the length of the old value is less than the start index, pad the old value with spaces until it reaches the start index
    if old_value_len < num {
        let padding = " ".repeat(num - old_value_len);
        value_with_capacity.push_str(&old_value);
        value_with_capacity.push_str(&padding);
    } else {
        // Otherwise, truncate the old value at the start index
        let truncate_value = &old_value[..num];
        value_with_capacity.push_str(truncate_value);
    }

    // Append the new value to the old value
    value_with_capacity.push_str(&new_value);

    // Calculate the length of the resulting string
    let len = value_with_capacity.len();

    // Set the new value in the database
    db.set(key.to_string(), DataType::String(value_with_capacity));

    // Return the length of the new value as a string
    len.to_string()
}
