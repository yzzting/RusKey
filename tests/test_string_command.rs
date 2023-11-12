use rus_key::db::Db;
use rus_key::db::DataType;
use rus_key::func::string::StringCommand;
use rus_key::func::expired::ExpiredCommand;
use rus_key::command_factory::Command;

fn ttl_command(db: &mut Db, command: &str, key: &str) -> i64 {
    let command_ttl = ExpiredCommand::new(command.to_string());
    let command_ttl_str = key;
    let mut parts_ttl = command_ttl_str.split_ascii_whitespace();
    let result = command_ttl.execute(&mut parts_ttl, db);
    result.unwrap().parse::<i64>().unwrap()
}

fn general_command(db: &mut Db, command_set: &StringCommand, command_set_str: &str) -> String {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    return result_set.unwrap();
}

fn assert(db: &mut Db, key: &str, value: &str) {
    assert_eq!(
        match db.get(key) {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        value
    );
}

#[test]
fn test_set_command() {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());
    let result = general_command(&mut db, &command, "key value");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key", "value");

    let result = general_command(&mut db, &command, "key value1 value2");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key", "value1");

    let result = general_command(&mut db, &command, "key \"This is value\"");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key", "This is value");

    let result = general_command(&mut db, &command, "key_arg value EX 60");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key_arg", "value");

    let ttl_key_result = ttl_command(&mut db, "ttl", "key_arg");
    assert!(0 < ttl_key_result && ttl_key_result <= 60);
}

#[test]
fn test_get_command() {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let set_result = general_command(&mut db, &set_command, "key value");
    assert_eq!(set_result, "OK".to_string());

    let get_command = StringCommand::new("get".to_string());
    let get_result = general_command(&mut db, &get_command, "key");
    assert_eq!(get_result, "value".to_string());
}

#[test]
fn test_getrange_command() {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());
    let result = general_command(&mut db, &command, "key \"This is a string\"");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key", "This is a string");

    let command_getrange = StringCommand::new("getrange".to_string());
    
    let result = general_command(&mut db, &command_getrange, "key 0 3");
    assert_eq!(result, "This".to_string());

    let result = general_command(&mut db, &command_getrange, "key 0 16");
    assert_eq!(result, "This is a string".to_string());

    let result = general_command(&mut db, &command_getrange, "key 0 -1");
    assert_eq!(result, "This is a string".to_string());

    let result = general_command(&mut db, &command_getrange, "key -3 -1");
    assert_eq!(result, "ing".to_string());

    let result = general_command(&mut db, &command_getrange, "key 10 100");
    assert_eq!(result, "string".to_string());

    let result = general_command(&mut db, &command_getrange, "key 0 0");
    assert_eq!(result, "T".to_string());

    let result = general_command(&mut db, &command_getrange, "key 2 1");
    assert_eq!(result, "".to_string());

    let result = general_command(&mut db, &command_getrange, "key -20 -1");
    assert_eq!(result, "This is a string".to_string());

    let result = general_command(&mut db, &command_getrange, "key -20 -19");
    assert_eq!(result, "T".to_string());
    
    let result = general_command(&mut db, &command_getrange, "non_existent_key 0 -2");
    assert_eq!(result, "".to_string());
}