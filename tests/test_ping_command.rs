use ping_commands::ping::PingCommand;
use rus_key_trait::command_trait::Command;
use rus_key_lib::db::Db;
#[test]
fn test_ping_command() {
    let mut db = Db::new();
    let command = PingCommand {};
    let command_str = "";
    let mut parts = command_str.split_ascii_whitespace();

    let result = command.execute(&mut parts, &mut db);

    assert_eq!(result.unwrap(), "PONG".to_string());
}

#[test]
fn test_ping_value_command() {
    let mut db = Db::new();
    let command = PingCommand {};
    let command_str = "test_ping";
    let mut parts = command_str.split_ascii_whitespace();

    let result = command.execute(&mut parts, &mut db);

    assert_eq!(result.unwrap(), "test_ping".to_string());
}
