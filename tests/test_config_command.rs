use rus_key_trait::command_trait::Command;
use rus_key_db::db::{DataType, Db};
use config_commands::config::ConfigCommand;
use rus_key_lib::init;

#[test]
fn test_config_get_command() {
    let mut db = Db::new();
    let config_map = init::init();
    db.set(
        "ruskey_config".to_string(),
        DataType::ZSet(config_map.clone()),
    );

    let command = ConfigCommand {};
    let all_command_str = "get *";
    let mut all_parts = all_command_str.split_ascii_whitespace();
    let all_result = command.execute(&mut all_parts, &mut db);
    assert_eq!(
        all_result.unwrap(),
        config_map
            .iter()
            .map(|(filed, value)| format!("{}: {}", filed, value))
            .collect::<Vec<String>>()
            .join(" ")
    );

    let single_command_str = "get host";
    let mut single_parts = single_command_str.split_ascii_whitespace();
    let single_result = command.execute(&mut single_parts, &mut db);
    assert_eq!(single_result.unwrap(), "host: 127.0.0.1".to_string());
}

// TODO: test set command, but it will modify config file, so it's not good
// Currently, only the port and host configurations cannot be modified
// #[test]
// fn test_config_set_command() {
// let mut db = Db::new();
// let config_map = init::init();
// db.set("ruskey_config".to_string(), DataType::ZSet(config_map.clone()));

// let command = ConfigCommand {};
// let command_str = "set port 16378";
// let mut parts = command_str.split_ascii_whitespace();
// let result = command.execute(&mut parts, &mut db);
// assert_eq!(result.unwrap(), "Cannot modify".to_string());

// let command_get = "get port";
// let mut parts_get = command_get.split_ascii_whitespace();
// let result_get = command.execute(&mut parts_get, &mut db);
// assert_eq!(result_get.unwrap(), "port: 16378".to_string());
// }
