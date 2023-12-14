use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use crate::r#const::EMPTY;
use rus_key_command_lib::get_parts;

/// Retrieves and deletes a value from the database.
///
/// This function is designed to retrieve and delete a value from a database. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database from which the value is retrieved and deleted.
///
/// The function first retrieves the key from the `parts` iterator. If no key is provided, it returns an error message.
///
/// Next, it retrieves the value associated with the key from the database. If the value is `EMPTY`, it returns `EMPTY`.
///
/// Finally, it deletes the key-value pair from the database and returns the value.
pub fn get_del(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, _) = get_parts(parts, false);
    if key.is_empty() {
        return "GetDel Error: Key not specified".to_string();
    }
    let value = get(false, parts, &key, db);
    if value == EMPTY {
        return EMPTY.to_string();
    }
    db.delete(&key);
    value
}
