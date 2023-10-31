use rus_key::db::{Db, DataType};
use rus_key::init;
use rus_key::func::config::ConfigCommand;
use rus_key::command_factory::Command;

#[test]
fn test_config_get_command() {
    let mut db = Db::new();
    let config_map = init::init();
    db.set("ruskey_config".to_string(), DataType::ZSet(config_map.clone()));

    let command = ConfigCommand {};
    let all_command_str = "get *";
    let mut all_parts = all_command_str.split_ascii_whitespace();
    let all_result = command.execute(&mut all_parts, &mut db);
    assert_eq!(all_result.unwrap(), config_map.iter()
        .map(|(filed, value)| format!("{}: {}", filed, value))
        .collect::<Vec<String>>()
        .join(" ")
    );

    let single_command_str = "get host";
    let mut single_parts = single_command_str.split_ascii_whitespace();
    let single_result = command.execute(&mut single_parts, &mut db);
    assert_eq!(single_result.unwrap(), "host: 127.0.0.1".to_string());
}

// TODO: test set command
#[test]
fn test_config_set_command() {
    
}