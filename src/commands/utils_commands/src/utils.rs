use std::str::SplitAsciiWhitespace;

use rus_key_trait::command_trait::Command;
use rus_key_db::db::Db;

use crate::check_expired::check_expired;
use crate::rename::rename;
use crate::randomkey::randomkey;
use crate::del_key::del_key;
use crate::check_type::check_type;

pub struct UtilsCommand {
    command: String,
}

impl UtilsCommand {
    pub fn new(command: String) -> UtilsCommand {
        UtilsCommand { command }
    }
}

impl Command for UtilsCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "exists" => Ok(check_expired(parts.next(), db)),
            "rename" => {
                let old_name = parts.next();
                let new_name = parts.next();
                let rename = rename(old_name, new_name, "", db);
                rename
            }
            "renamenx" => {
                let old_name = parts.next();
                let new_name = parts.next();
                let rename = rename(old_name, new_name, "nx", db);
                rename
            }
            "randomkey" => Ok(randomkey(db)),
            "del" => Ok(del_key(parts, db)),
            "type" => Ok(check_type(parts.next(), db)),
            _ => Err("UtilsCommand Error: Command not found"),
        }
    }
}
