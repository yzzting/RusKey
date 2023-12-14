use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::{get_parts, slice_from_end};

/// Retrieves a range of characters from the value of a specified key in the database.
///
/// This function takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database from which the value is retrieved.
///
/// The function first retrieves the key from the `parts` iterator. If no key is provided, it returns an error message.
///
/// Next, it retrieves the start and end indices for the range from the `parts` iterator. If either index is not provided or cannot be parsed as an `isize`, it returns an error message.
///
/// The function then retrieves the value associated with the key from the database. If the value is `EMPTY`, it returns `EMPTY`.
///
/// Finally, it slices the value from the start to the end index and returns the resulting string.
pub fn get_range(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve the key from the `parts` iterator
    let (key, _) = get_parts(parts, false);
    if key.is_empty() {
        return "GetRange Error: Key not specified".to_string();
    }
    // Retrieve the start index from the `parts` iterator
    let start = match parts.next() {
        Some(start_str) => match start_str.parse::<isize>() {
            Ok(start) => start,
            Err(_) => return "GetRange Error: Invalid start value".to_string(),
        },
        None => return "GetRange Error: Start not specified".to_string(),
    };
    // Retrieve the end index from the `parts` iterator
    let end = match parts.next() {
        Some(end_str) => match end_str.parse::<isize>() {
            Ok(end) => end,
            Err(_) => return "GetRange Error: Invalid end value".to_string(),
        },
        None => return "GetRange Error: End not specified".to_string(),
    };
    // Retrieve the value associated with the key from the database
    let key_value = get(false, parts, &key, db);
    if key_value == EMPTY {
        return EMPTY.to_string();
    }
    // Slice the value from the start to the end index and return the resulting string
    return slice_from_end(&key_value, start, end);
}
