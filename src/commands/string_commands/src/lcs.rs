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
    let (lcs_str, lcs_len) = fn_lcs(&value1, &value2);
    let mut result_str = lcs_str;
    let mut err_count = 0;
    while let Some(arg) = parts.next() {
        let lower_arg = arg.to_lowercase();
        match lower_arg.as_str() {
            "len" => {
                err_count += 1;
                result_str = lcs_len.to_string();
            }
            // TODO add idx The implementation of idx is too difficult
            _ => {}
        }
    }

    // IDX and LEN not be able to exist in the same time
    if err_count > 1 {
        return "ERR wrong number of arguments for command".to_string();
    }

    result_str
}
