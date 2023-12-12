use crate::command_trait::Command;
use crate::db::Db;
use std::str::SplitAsciiWhitespace;

pub struct PingCommand {}

impl Command for PingCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        _db: &mut Db,
    ) -> Result<String, &'static str> {
        let arg = parts.next();
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => Ok("PONG".to_string()),
        }
    }
}
