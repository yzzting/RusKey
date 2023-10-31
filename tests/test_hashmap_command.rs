use rus_key::db::Db;
use rus_key::func::hashmap::HashMapCommand;
use rus_key::command_factory::Command;

#[test]
fn test_hmset_command() {
    let mut db = Db::new();
    let command = HashMapCommand::new("hmset".to_string());
    let command_str = "obj field value";
    let mut parts = command_str.split_ascii_whitespace();

    let result = command.execute(&mut parts, &mut db);

    assert_eq!(result.unwrap(), "OK".to_string());
}

#[test]
fn test_hgetall_command() {
    let mut db = Db::new();
    let hmset_command = HashMapCommand::new("hmset".to_string());
    let hmset_command_str = "obj field value";
    let mut hmset_parts = hmset_command_str.split_ascii_whitespace();
    let hmset_result = hmset_command.execute(&mut hmset_parts, &mut db);
    assert_eq!(hmset_result, Ok("OK".to_string()));

    let hgetall_command = HashMapCommand::new("hgetall".to_string());
    let hgetall_command_str = "obj";
    let mut hgetall_parts = hgetall_command_str.split_ascii_whitespace();
    let hgetall_result = hgetall_command.execute(&mut hgetall_parts, &mut db);
    assert_eq!(hgetall_result, Ok("field: value".to_string()));
}