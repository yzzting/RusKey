use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use rus_key_command_lib::get_value;

/// Sets the values of multiple keys in the database.
///
/// This function is designed to set the values of multiple keys in a database. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database where the values are set.
///
/// The function first retrieves all the key-value pairs from the `parts` iterator and stores them in a vector. If a key is provided without a corresponding value, it returns an error message.
///
/// Next, it iterates over the vector of key-value pairs and sets each value in the database.
///
/// Finally, it returns "OK" to indicate that the operation was successful.
pub fn mset(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve all the key-value pairs from the `parts` iterator and store them in a vector
    let mut key_value_vec: Vec<(String, String)> = Vec::new();
    while let Some(key) = parts.next() {
        let key = get_value(key.to_string(), parts);
        let value = match parts.next() {
            Some(value) => value.to_string(),
            None => return "wrong number of arguments for 'mset' command".to_string(),
        };
        let value = get_value(value, parts);
        key_value_vec.push((key.to_string(), value));
    }
    // Iterate over the vector of key-value pairs and set each value in the database
    for (key, value) in key_value_vec {
        db.set(key.to_string(), DataType::String(value.to_string()));
    }
    // Return "OK" to indicate that the operation was successful
    "OK".to_string()
}
