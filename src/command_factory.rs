use crate::db::Db;
use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;
use crate::func::string::StringCommand;
use crate::func::hashmap::HashMapCommand;
use crate::func::ping::PingCommand;
use crate::func::config::ConfigCommand;
use crate::func::utils::UtilsCommand;
use crate::func::expired::ExpiredCommand;

pub trait Command: Send + Sync {
    fn execute(&self, parts: &mut SplitAsciiWhitespace, db: &mut Db) -> Result<String, &'static str>;
}

pub struct CommandFactory {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandFactory {
    pub fn new() -> CommandFactory {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert("ping".to_string(), Box::new(PingCommand {}));

        // String
        commands.insert("append".to_string(), Box::new(StringCommand::new("append".to_string())));
        commands.insert("decr".to_string(), Box::new(StringCommand::new("decr".to_string())));
        commands.insert("decrby".to_string(), Box::new(StringCommand::new("decrby".to_string())));
        commands.insert("getdel".to_string(), Box::new(StringCommand::new("getdel".to_string())));
        commands.insert("set".to_string(), Box::new(StringCommand::new("set".to_string())));
        commands.insert("get".to_string(), Box::new(StringCommand::new("get".to_string())));
        commands.insert("getrange".to_string(), Box::new(StringCommand::new("getrange".to_string())));

        // HashMap
        commands.insert("hmset".to_string(), Box::new(HashMapCommand::new("hmset".to_string())));
        commands.insert("hgetall".to_string(), Box::new(HashMapCommand::new("hgetall".to_string())));

        // Config
        commands.insert("config".to_string(), Box::new(ConfigCommand::new()));
        // Expired
        commands.insert("expired".to_string(), Box::new(ExpiredCommand::new("expired".to_string())));
        commands.insert("expireat".to_string(), Box::new(ExpiredCommand::new("expireat".to_string())));
        commands.insert("pexpireat".to_string(), Box::new(ExpiredCommand::new("pexpireat".to_string())));
        commands.insert("ttl".to_string(), Box::new(ExpiredCommand::new("ttl".to_string())));
        commands.insert("pttl".to_string(), Box::new(ExpiredCommand::new("pttl".to_string())));
        commands.insert("persist".to_string(), Box::new(ExpiredCommand::new("persist".to_string())));

        // Utils
        commands.insert("rename".to_string(), Box::new(UtilsCommand::new("rename".to_string())));
        commands.insert("renamenx".to_string(), Box::new(UtilsCommand::new("renamenx".to_string())));
        commands.insert("randomkey".to_string(), Box::new(UtilsCommand::new("randomkey".to_string())));
        commands.insert("del".to_string(), Box::new(UtilsCommand::new("del".to_string())));
        commands.insert("exists".to_string(), Box::new(UtilsCommand::new("exists".to_string())));
        commands.insert("type".to_string(), Box::new(UtilsCommand::new("type".to_string())));
        CommandFactory { commands }
    }

    pub fn create(&self, cmd: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(cmd)
    }
}