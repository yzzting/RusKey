use rus_key_command_lib::{fn_lcs, get_parts};
use rus_key_db::db::Db;
use std::str::SplitAsciiWhitespace;

use crate::get::get;

pub fn lcs(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key1, key2) = get_parts(parts, true);

    // If either key is empty, return an error.
    if key1.is_empty() || key2.is_empty() {
        return "ERR wrong number of arguments for command".to_string();
    }

    let value1 = get(false, parts, &key1, db);
    let value2 = get(false, parts, &key2, db);

    // If either value is empty, return an empty string.
    if value1.is_empty() || value2.is_empty() {
        return "".to_string();
    }

    // Call the fn_lcs function from the command_lib crate.
    fn_lcs(&value1, &value2)

    // TODO handle extra arguments
}
