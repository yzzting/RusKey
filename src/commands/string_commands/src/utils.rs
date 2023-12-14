use rus_key_db::db::Db;
use expired_commands::expired::ExpiredCommand;
use rus_key_trait::command_trait::Command;

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

