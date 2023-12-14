use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;
use crate::get::get;
use rus_key_command_lib::get_value;

/// Retrieves the values of multiple keys from the database.
///
/// This function is designed to retrieve the values of multiple keys from a database. It takes two parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database from which the values are retrieved.
///
/// The function first retrieves all the keys from the `parts` iterator and stores them in a vector.
///
/// Next, it iterates over the vector of keys, retrieves the value associated with each key from the database, and stores the values in a new vector.
///
/// Finally, it joins the values in the vector into a single string, with each value separated by a space, and returns this string.
pub fn mget(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    // Retrieve all the keys from the `parts` iterator and store them in a vector
    let mut key_vec: Vec<String> = Vec::new();
    while let Some(key) = parts.next() {
        let key = get_value(key.to_string(), parts);
        key_vec.push(key.to_string());
    }
    // Iterate over the vector of keys, retrieve the value associated with each key from the database, and store the values in a new vector
    let mut value_vec: Vec<String> = Vec::new();
    for key in key_vec {
        let value = get(false, parts, &key, db);
        value_vec.push(value);
    }
    // Join the values in the vector into a single string, with each value separated by a space, and return this string
    value_vec.join(" ")
}
