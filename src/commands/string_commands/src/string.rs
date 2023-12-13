use rus_key_trait::command_trait::Command;
use rus_key_db::db::Db;
use crate::append::append;
use crate::utils::handle_accumulation;
use crate::incrby_float::incrby_float;
use crate::get::get;
use crate::get_del::get_del;
use crate::get_ex::get_ex;
use crate::get_range::get_range;
use crate::get_set::get_set;
use crate::set::set;
use crate::mset::mset;
use crate::mget::mget;
use crate::set_gange::set_gange;
use crate::str_len::str_len;
use crate::r#const::Accumulation;
use std::str::SplitAsciiWhitespace;

pub struct StringCommand {
    command: String,
}

impl StringCommand {
    pub fn new(command: String) -> StringCommand {
        StringCommand { command }
    }
}

impl Command for StringCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        match self.command.as_str() {
            "append" => Ok(append(parts, db)),
            "decr" => Ok(handle_accumulation(parts, db, Accumulation::Decr, false)),
            "decrby" => Ok(handle_accumulation(parts, db, Accumulation::Decr, true)),
            "get" => Ok(get(true, parts, "", db)),
            "getdel" => Ok(get_del(parts, db)),
            "getex" => Ok(get_ex(parts, db)),
            "incr" => Ok(handle_accumulation(parts, db, Accumulation::Incr, false)),
            "incrby" => Ok(handle_accumulation(parts, db, Accumulation::Incr, true)),
            "incrbyfloat" => Ok(incrby_float(parts, db)),
            "getrange" => Ok(get_range(parts, db)),
            "getset" => Ok(get_set(parts, db)),
            "set" => Ok(set(parts, db)),
            "mset" => Ok(mset(parts, db)),
            "mget" => Ok(mget(parts, db)),
            "setrange" => Ok(set_gange(parts, db)),
            "strlen" => Ok(str_len(parts, db)),
            _ => Err("StringCommand Error: Command not found"),
        }
    }
}
