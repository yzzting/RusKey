use crate::db::Db;
use crate::commands::config::ConfigCommand;
use crate::commands::expired::ExpiredCommand;
use crate::commands::hashmap::HashMapCommand;
use crate::commands::ping::PingCommand;
use crate::commands::string::StringCommand;
use crate::commands::utils::UtilsCommand;
use crate::init_commands::{EXPIRED_COMMANDS, HASHMAP_COMMANDS, STRING_COMMANDS, UTILS_COMMANDS};
use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;

pub trait Command: Send + Sync {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str>;
}

pub struct CommandFactory {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandFactory {
    pub fn new() -> CommandFactory {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        // Config
        commands.insert("config".to_string(), Box::new(ConfigCommand::new()));
        // Ping
        commands.insert("ping".to_string(), Box::new(PingCommand {}));

        // String
        for command in STRING_COMMANDS.iter() {
            commands.insert(
                command.to_string(),
                Box::new(StringCommand::new(command.to_string())),
            );
        }

        // HashMap
        for command in HASHMAP_COMMANDS.iter() {
            commands.insert(
                command.to_string(),
                Box::new(HashMapCommand::new(command.to_string())),
            );
        }

        // Expired
        for command in EXPIRED_COMMANDS.iter() {
            commands.insert(
                command.to_string(),
                Box::new(ExpiredCommand::new(command.to_string())),
            );
        }

        // Utils
        for command in UTILS_COMMANDS.iter() {
            commands.insert(
                command.to_string(),
                Box::new(UtilsCommand::new(command.to_string())),
            );
        }
        CommandFactory { commands }
    }

    pub fn create(&self, cmd: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(cmd)
    }
}
