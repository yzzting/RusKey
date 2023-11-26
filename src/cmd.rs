use crate::db::Db;
use std::str::SplitAsciiWhitespace;

use crate::command_factory::CommandFactory;

pub fn handle_command(
    parts: &mut SplitAsciiWhitespace,
    db: &mut Db,
    factory: &CommandFactory,
) -> Result<String, &'static str> {
    let cmd = match parts.next() {
        Some(cmd) => cmd.to_lowercase(),
        None => return Err("No command"),
    };

    match factory.create(&cmd) {
        Some(command) => command.execute(parts, db),
        None => Err("Invalid command!"),
    }
}
