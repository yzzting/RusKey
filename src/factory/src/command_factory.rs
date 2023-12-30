use crate::command_init::{
    EXPIRED_COMMANDS, HASHMAP_COMMANDS, LIST_COMMANDS, STRING_COMMANDS, UTILS_COMMANDS,
};
use config_commands::config::ConfigCommand;
use expired_commands::expired::ExpiredCommand;
use hashmap_commands::hashmap::HashMapCommand;
use list_commands::list::ListCommand;
use ping_commands::ping::PingCommand;
use rus_key_trait::command_trait::Command;
use std::collections::HashMap;
use string_commands::string::StringCommand;
use utils_commands::utils::UtilsCommand;

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

        // List
        for command in LIST_COMMANDS.iter() {
            commands.insert(
                command.to_string(),
                Box::new(ListCommand::new(command.to_string())),
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
