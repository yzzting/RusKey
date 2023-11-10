use rus_key::db::Db;
use rus_key::db::DataType;
use rus_key::func::string::StringCommand;
use rus_key::command_factory::Command;

#[test]
fn test_set_command() {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());
    let command_str = "key value";
    let mut parts = command_str.split_ascii_whitespace();

    let result = command.execute(&mut parts, &mut db);

    assert_eq!(result.unwrap(), "OK".to_string());
    assert_eq!(
        match db.get("key") {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        "value"
    );

    let command_str = "key value1 value2";
    let mut parts = command_str.split_ascii_whitespace();

    let result = command.execute(&mut parts, &mut db);

    assert_eq!(result.unwrap(), "OK".to_string());
    assert_eq!(
        match db.get("key") {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        "value1"
    );

    let command_str = "key \"This is value\"";
    let mut parts = command_str.split_ascii_whitespace();
    let result = command.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "OK".to_string());
    assert_eq!(
        match db.get("key") {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        "This is value"
    );
}

#[test]
fn test_get_command() {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let set_command_str = "key value";
    let mut set_parts = set_command_str.split_ascii_whitespace();
    let set_result = set_command.execute(&mut set_parts, &mut db);
    assert_eq!(set_result, Ok("OK".to_string()));

    let get_command = StringCommand::new("get".to_string());
    let get_command_str = "key";
    let mut get_parts = get_command_str.split_ascii_whitespace();
    let get_result = get_command.execute(&mut get_parts, &mut db);
    assert_eq!(get_result, Ok("value".to_string()));
}

#[test]
fn test_getrange_command() {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());
    let command_str = "key \"This is a string\"";
    let mut parts = command_str.split_ascii_whitespace();
    let result = command.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "OK".to_string());
    assert_eq!(
        match db.get("key") {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        "This is a string"
    );

    let command_getrange = StringCommand::new("getrange".to_string());
    let command_getrange_str = "key 0 3";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "This".to_string());

    let command_getrange_str = "key 0 16";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "This is a string".to_string());

    let command_getrange_str = "key 0 -1";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "This is a string".to_string());

    let command_getrange_str = "key -3 -1";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "ing".to_string());

    let command_getrange_str = "key 10 100";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "string".to_string());

    let command_getrange_str = "key 0 0";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "T".to_string());

    let command_getrange_str = "key 2 1";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "".to_string());

    let command_getrange_str = "key -20 -1";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "This is a string".to_string());

    let command_getrange_str = "key -20 -19";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "T".to_string());

    let command_getrange_str = "non_existent_key 0 -2";
    let mut parts = command_getrange_str.split_ascii_whitespace();
    let result = command_getrange.execute(&mut parts, &mut db);
    assert_eq!(result.unwrap(), "".to_string());
}