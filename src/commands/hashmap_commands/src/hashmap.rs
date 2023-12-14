use std::str::SplitAsciiWhitespace;
use rus_key_trait::command_trait::Command;
use rus_key_db::db::Db;
use crate::hgetall::hgetall;
use crate::hmset::hmset;

pub struct HashMapCommand {
    command: String,
}

impl HashMapCommand {
    pub fn new(command: String) -> HashMapCommand {
        HashMapCommand { command }
    }
}

impl Command for HashMapCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "hmset" => hmset(parts, db),
            "hgetall" => hgetall(parts, db),
            _ => Err("HashMapCommand Error: Command not found"),
        }
    }
}
