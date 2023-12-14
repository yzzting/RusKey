use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

/// Returns the length of the value of a specified key in the database.
///
/// This function is designed to return the length of the value of a specified key in a database. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database from which the value is retrieved.
///
/// The function first retrieves the key from the `parts` iterator. If no key is provided, it returns an error message.
///
/// Next, it retrieves the value associated with the key from the database. If the value is `EMPTY` or an empty string, it returns "0".
///
/// Finally, it returns the length of the value as a string.
pub fn str_len(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve the key from the `parts` iterator
    let (key, _) = get_parts(parts, false);
    if key.is_empty() {
        return "StrLen Error: Key not specified".to_string();
    }
    // Retrieve the value associated with the key from the database
    let key_value = get(false, parts, &key, db);
    // If the value is `EMPTY` or an empty string, return "0"
    if key_value == EMPTY || key_value.is_empty() {
        return "0".to_string();
    }
    // Return the length of the value as a string
    key_value.as_str().len().to_string()
}
