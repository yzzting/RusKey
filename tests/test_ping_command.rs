use rus_key::command_factory::Command;
use rus_key::db::Db;
use rus_key::commands::ping::PingCommand;

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
