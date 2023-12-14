use std::str::SplitAsciiWhitespace;
use rus_key_db::db::{DataType, Db};
use crate::get::get;
use crate::r#const::{Accumulation, EMPTY};
use rus_key_command_lib::{get_parts, is_integer};

/// Handles the accumulation of values in a database.
///
/// This function is designed to handle the accumulation of values in a database. It takes four parameters:
/// - `parts`: A mutable reference to a `SplitAsciiWhitespace` iterator. This is used to parse the command and its arguments.
/// - `db`: A mutable reference to a `Db` instance. This is the database on which the command is executed.
/// - `accumulation`: An `Accumulation` enum value. This determines whether the operation is an increment (`Incr`) or decrement (`Decr`).
/// - `is_by`: A boolean value. If true, the function expects a numeric value as the next part of the command.
///
/// The function first determines whether the operation is an increment or decrement based on the `accumulation` parameter. It then retrieves the key from the `parts` iterator. If no key is provided, it returns an error message.
///
/// Next, it checks if the `is_by` flag is true. If it is, it attempts to parse the next part of the command as an `i128` integer. If the parsing fails or if `is_by` is false, it defaults to 1.
///
/// The function then retrieves the old value associated with the key from the database. If the old value is `EMPTY` or if it's not an integer, it sets the new value to the `accumulation_str` (which is 1 for increment and -1 for decrement). Otherwise, it calculates the new value by adding or subtracting the `num_value` from the old value, depending on the `accumulation_str`.
///
/// Finally, it sets the new value in the database and returns it as a string.
pub fn handle_accumulation(
    parts: &mut SplitAsciiWhitespace,
    db: &mut Db,
    accumulation: Accumulation,
    is_by: bool,
) -> String {
    // match accumulation incr or decr
    let accumulation_str = match accumulation {
        Accumulation::Incr => 1,
        Accumulation::Decr => -1,
    };
    let (key, _) = get_parts(parts, false);
    if key.is_empty() {
        return "ERR wrong number of arguments for command".to_string();
    }
    // is_by true get num value
    let num: Option<i128> = if is_by {
        match parts.next() {
            Some(n) => n.parse::<i128>().ok(),
            None => None,
        }
    } else {
        Some(1)
    };

    let num_value = match num {
        Some(n) => n,
        None => return "ERR wrong number of arguments for command".to_string(),
    };

    let old_value = get(false, parts, &key, db);

    // old_value is nil
    let new_value = if old_value == EMPTY {
        // accumulation_str == Incr is 1 or Decr is -1
        accumulation_str
    } else {
        // check if old_value is not an integer
        if !is_integer(&old_value) {
            return "Value is not an integer or out of range".to_string();
        }
        // old_value is an integer
        match old_value.parse::<i128>() {
            Ok(n) => {
                println!("n: {}, num_value: {}", n, num_value);
                // accumulation_str == Incr is n + num_value or Decr is n - num_value
                let new_value = n + accumulation_str * num_value;
                // (n + num_value) as i64
                if new_value < i64::MIN as i128 || new_value > i64::MAX as i128 {
                    return "Value is not an integer or out of range".to_string();
                }
                new_value
            }
            Err(_) => return "Value is not an integer or out of range".to_string(),
        }
    };

    db.set(key.to_string(), DataType::String(new_value.to_string()));
    new_value.to_string()
}
