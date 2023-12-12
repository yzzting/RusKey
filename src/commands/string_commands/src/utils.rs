use std::str::SplitAsciiWhitespace;
use rus_key_lib::db::Db;
use expired_commands::expired::ExpiredCommand;

/// Checks if a given string starts and ends with quotation marks.
///
/// This function takes a string and returns a boolean value indicating whether the string starts and ends with quotation marks.
/// It does this by checking the first and last character of the string.
/// If the first character is a quotation mark and the last character is not a quotation mark, it returns true. If not, it returns false.
///
/// # Arguments
///
/// * `s` - The string to be checked.
///
/// # Returns
///
/// * A boolean value indicating whether the string starts and ends with quotation marks.
pub fn is_with_quotation_marks(s: &str) -> bool {
    s.starts_with('"') && !s.ends_with('"')
}

/// Executes a given command on the database.
///
/// This function takes a mutable reference to the database, a command to execute, and a string representation of the command.
/// It splits the command string into parts using whitespace as a delimiter.
/// Then it executes the command on the database using the parts.
/// If the command execution is successful, it returns the result. If not, it unwraps the error.
///
/// # Arguments
///
/// * `db` - A mutable reference to the database.
/// * `command_set` - The command to be executed.
/// * `command_set_str` - The string representation of the command.
///
/// # Returns
///
/// * A string representing the result of the command execution.
pub fn general_command(db: &mut Db, command_set: &ExpiredCommand, command_set_str: &str) -> String {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    return result_set.unwrap();
}

/// Checks if a given string is an integer.
///
/// This function takes a string and attempts to parse it into an i64 integer.
/// If the parsing is successful, it returns true. If not, it returns false.
///
/// # Arguments
///
/// * `s` - The string to be checked.
///
/// # Returns
///
/// * A boolean value indicating whether the string is an integer.
pub fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

/// Checks if a given string is a number.
///
/// This function takes a string and attempts to parse it into a f64 float.
/// If the parsing is successful, it returns true. If not, it returns false.
///
/// # Arguments
///
/// * `s` - The string to be checked.
///
/// # Returns
///
/// * A boolean value indicating whether the string is a number.
pub fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

/// Retrieves the key and value from a SplitAsciiWhitespace iterator.
///
/// This function takes a mutable reference to a SplitAsciiWhitespace iterator and a boolean indicating whether to retrieve a value.
/// It retrieves the next part from the iterator as the key and calls the get_value function to process the key.
/// If the boolean is true, it retrieves the next part from the iterator as the value and calls the get_value function to process the value.
/// If the boolean is false, it sets the value to an empty string.
///
/// # Arguments
///
/// * `parts` - A mutable reference to a SplitAsciiWhitespace iterator.
/// * `is_value` - A boolean indicating whether to retrieve a value.
///
/// # Returns
///
/// * A tuple containing the processed key and value.
pub fn get_parts(parts: &mut SplitAsciiWhitespace, is_value: bool) -> (String, String) {
    let key = match parts.next() {
        Some(key) => key.to_string(),
        None => "".to_string(),
    };
    let key = get_value(key, parts);
    let value = if is_value {
        match parts.next() {
            Some(value) => get_value(value.to_string(), parts),
            None => "".to_string(),
        }
    } else {
        "".to_string()
    };
    (key, value)
}

/// Retrieves the value from a string, considering quotation marks.
///
/// This function takes a string and a mutable reference to a SplitAsciiWhitespace iterator.
/// If the string starts with a quotation mark, it continues to concatenate the parts from the iterator to the string until it finds a part that ends with a quotation mark.
/// After that, it trims the quotation marks from the start and end of the string.
///
/// # Arguments
///
/// * `value` - The initial string to be processed.
/// * `parts` - A mutable reference to a SplitAsciiWhitespace iterator.
///
/// # Returns
///
/// * A string that has been processed to include parts within quotation marks and has had its quotation marks removed.
pub fn get_value(value: String, parts: &mut SplitAsciiWhitespace) -> String {
    let mut value = value;
    if is_with_quotation_marks(value.as_str()) {
        while let Some(part) = parts.next() {
            value.push_str(" ");
            value.push_str(part);
            if part.ends_with('"') {
                break;
            }
        }
    }
    value = value.trim_matches('"').to_string();
    value
}

/// Returns a substring from a given string, starting and ending at the specified indices.
///
/// This function takes a string and two indices (start and end) as arguments.
/// It returns a substring that starts at the start index and ends at the end index.
/// If the start index is greater than the end index, it returns an empty string.
/// If the start or end index is negative, it is treated as an offset from the end of the string.
///
/// # Arguments
///
/// * `str` - The string from which to extract the substring.
/// * `start` - The start index for the substring. If negative, it is treated as an offset from the end of the string.
/// * `end` - The end index for the substring. If negative, it is treated as an offset from the end of the string.
///
/// # Returns
///
/// * A string that is a substring of the original string, starting at the start index and ending at the end index.
pub fn slice_from_end(str: &str, start: isize, end: isize) -> String {
    if start > end {
        return "".to_string();
    }
    let char_vec: Vec<char> = str.chars().collect();
    let char_vec_len = char_vec.len() as isize;
    let (start, end) = if start < 0 || end < 0 {
        (
            (char_vec_len + start).max(0) as usize,
            (char_vec_len + end + 1).max(0) as usize,
        )
    } else {
        (start as usize, (end + 1).min(char_vec_len) as usize)
    };
    char_vec[start..end].iter().collect::<String>()
}
