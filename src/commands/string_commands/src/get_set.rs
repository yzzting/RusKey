use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

/// Retrieves the old value of a key from the database and sets it to a new value.
///
/// This function is designed to retrieve the old value of a key from a database and set it to a new value. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database from which the old value is retrieved and where the new value is set.
///
/// The function first retrieves the key and the new value from the `parts` iterator. If no key or value is provided, it returns an error message.
///
/// Next, it retrieves the old value associated with the key from the database.
///
/// The function then sets the new value in the database.
///
/// Finally, it checks if the old value was `EMPTY`. If it was, it returns `EMPTY`. Otherwise, it returns the old value.
pub fn get_set(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve the key and the new value from the `parts` iterator
    let (key, value) = get_parts(parts, true);
    if key.is_empty() && value.is_empty() {
        return "GetSet Error: Key and value not specified".to_string();
    }
    // Retrieve the old value associated with the key from the database
    let old_value = get(false, parts, &key, db);
    // Set the new value in the database
    db.set(key.to_string(), DataType::String(value.to_string()));
    // If the old value was `EMPTY`, return `EMPTY`. Otherwise, return the old value.
    if old_value == EMPTY {
        EMPTY.to_string()
    } else {
        old_value
    }
}
