use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

/// Append function
///
/// This function takes a mutable reference to a `SplitAsciiWhitespace` object and a mutable reference to a `Db` object.
/// It retrieves the key and value from the `SplitAsciiWhitespace` object, and then appends the value to the old value associated with the key in the database.
/// If the key or value is empty, the function immediately returns an error message.
/// If the old value associated with the key is empty, the function directly uses the new value.
/// Otherwise, the function appends the new value to the old value and stores the result in the database.
/// The function finally returns the length of the new value.
///
/// # Arguments
///
/// * `parts` - A mutable reference to a `SplitAsciiWhitespace` object containing the key and value.
/// * `db` - A mutable reference to the database object.
///
/// # Returns
///
/// * `String` - The length of the new value, or an error message.

pub fn append(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, value) = get_parts(parts, true);
    if key.is_empty() && value.is_empty() {
        return "Append Error: Key or value not specified".to_string();
    }

    let old_value = get(false, parts, &key, db);
    let new_value = if old_value == EMPTY {
        value
    } else {
        let mut combined = old_value;
        combined.push_str(&value);
        combined
    };

    let len = new_value.len();
    db.set(key, DataType::String(new_value));
    len.to_string()
}
