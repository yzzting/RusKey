use std::str::SplitAsciiWhitespace;

use rus_key_trait::command_trait::Command;
use rus_key_db::db::{Db, DataType};

const CANNOT_MODIFY: [&str; 2] = ["port", "host"];

fn get_next_arg(parts: &mut SplitAsciiWhitespace) -> Result<String, &'static str> {
    match parts.next() {
        Some(arg) => Ok(arg.to_lowercase()),
        None => Err("No command"),
    }
}

// pub fn handle_config(parts: &mut SplitAsciiWhitespace, db: &mut dyn Db) -> Result<String, &'static str> {
//     let arg = get_next_arg(parts)?;
//     match arg.as_str() {
//         "get" => {
//             let value = get_next_arg(parts)?;
//             let btree_map = match db.get("ruskey_config") {
//                 Some(DataType::HashMap(btree_map)) => btree_map,
//                 _ => return Err("No such key or wrong data type"),
//             };
//             let result = if value == "*" {
//                 btree_map.iter()
//                     .map(|(filed, value)| format!("{}: {}", filed, value))
//                     .collect::<Vec<String>>()
//                     .join(" ")
//             } else {
//                 match btree_map.get(&value) {
//                     Some(match_value) => format!("{}: {}", value, match_value),
//                     None => return Err("No such key or wrong data type"),
//                 }
//             };

//             Ok(result.trim().to_string())
//         },
//         "set" => {
//             let field = get_next_arg(parts)?;
//             let value = get_next_arg(parts)?;
//             let mut btree_map = match db.get("ruskey_config") {
//                 Some(DataType::HashMap(btree_map)) => btree_map.clone(),
//                 _ => return Err("No such key or wrong data type"),
//             };
//             let keys: Vec<&String> = btree_map.keys().collect();
//             if !keys.contains(&&field) {
//                 return Err("No such key or wrong data type");
//             }
//             // check cannot modify
//             if CANNOT_MODIFY.contains(&field.as_str()) {
//                 return Err("Cannot modify");
//             }
//             btree_map.insert(field.to_string(), value.to_string());
//             db.set("ruskey_config".to_string(), DataType::HashMap(btree_map));
//             Ok("OK".to_string())
//         },
//         _ => Err("Config Invalid command!"),
//     }
// }

pub struct ConfigCommand {}

impl ConfigCommand {
    pub fn new() -> ConfigCommand {
        ConfigCommand {}
    }

    fn get(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        let value = get_next_arg(parts)?;
        let btree_map = match db.get("ruskey_config") {
            Some(DataType::ZSet(btree_map)) => btree_map,
            _ => return Err("No ruskey_config key"),
        };
        let result = if value == "*" {
            btree_map
                .iter()
                .map(|(filed, value)| format!("{}: {}", filed, value))
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            match btree_map.get(&value) {
                Some(match_value) => format!("{}: {}", value, match_value),
                None => return Err("No such key or wrong data type"),
            }
        };

        Ok(result.trim().to_string())
    }

    fn set(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str> {
        let field = get_next_arg(parts)?;
        let value = get_next_arg(parts)?;
        let mut btree_map = match db.get("ruskey_config") {
            Some(DataType::ZSet(btree_map)) => btree_map.clone(),
            _ => return Err("No such key or wrong data type"),
        };
        let keys: Vec<&String> = btree_map.keys().collect();
        if !keys.contains(&&field) {
            return Err("No such key or wrong data type");
        }
        // check cannot modify
        if CANNOT_MODIFY.contains(&field.as_str()) {
            return Err("Cannot modify");
        }
        btree_map.insert(field.to_string(), value.to_string());
        db.set("ruskey_config".to_string(), DataType::ZSet(btree_map));
        Ok("OK".to_string())
    }
}

impl Command for ConfigCommand {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str> {
        let arg = get_next_arg(parts)?;
        match arg.as_str() {
            "get" => self.get(parts, db),
            "set" => self.set(parts, db),
            _ => Err("Config Invalid command!"),
        }
    }
}
