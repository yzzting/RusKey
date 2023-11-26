use std::collections::HashSet;

const STRING_COMMANDS: [&str; 8] = ["append", "decr", "decrby", "get", "getdel", "getex", "getrange", "set"];
const HASHMAP_COMMANDS: [&str; 2] = ["hmset", "hgetall"];
const EXPIRED_COMMANDS: [&str; 6] = ["expired", "expireat", "pexpireat", "ttl", "pttl", "persist"];
const UTILS_COMMANDS: [&str; 6] = ["rename", "renamenx", "randomkey", "del", "exists", "type"];

pub fn init_commands() -> HashSet<String> {
    let mut commands_map: Vec<String> = Vec::new();

    commands_map.extend(STRING_COMMANDS.iter().map(|&s| s.to_string()));
    commands_map.extend(HASHMAP_COMMANDS.iter().map(|&s| s.to_string()));
    commands_map.extend(EXPIRED_COMMANDS.iter().map(|&s| s.to_string()));
    commands_map.extend(UTILS_COMMANDS.iter().map(|&s| s.to_string()));

    println!("{:?}", commands_map);

    commands_map.iter().cloned().collect::<HashSet<_>>()
}

