use crate::list_push::list_push;
use rus_key_db::db::Db;
use rus_key_trait::command_trait::Command;
use std::str::SplitAsciiWhitespace;

pub struct ListCommand {
    command: String,
}

impl ListCommand {
    pub fn new(command: String) -> ListCommand {
        ListCommand { command }
    }
}

impl Command for ListCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "lpush" => Ok(list_push(parts, db)),
            _ => Err("ListCommand Error: Command not found"),
        }
    }
}
