use rus_key::db::Db;
use rus_key::db::DataType;
use rus_key::func::string::StringCommand;
use rus_key::func::expired::ExpiredCommand;
use rus_key::command_factory::Command;

fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp_millis();
    timestamp
}

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

    // extra parameter
    // test expired time EX seconds
    let result = general_command(&mut db, &command, "key_ex_arg value EX 60");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key_ex_arg", "value");
    let ttl_key_ex_result = ttl_command(&mut db, "ttl", "key_ex_arg");
    assert!(0 < ttl_key_ex_result && ttl_key_ex_result <= 60);

    // test expired time PX milliseconds
    let result = general_command(&mut db, &command, "key_px_arg value PX 60000");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key_px_arg", "value");
    let ttl_key_px_result = ttl_command(&mut db, "ttl", "key_px_arg");
    assert!(0 < ttl_key_px_result && ttl_key_px_result <= 60);

    // error test
    // test expired time EX seconds
    let ex_result = general_command(&mut db, &command, "key_ex_arg value EX");
    assert_eq!(ex_result, "Set Error: Invalid expired time".to_string());

    // test expired time PX milliseconds
    let px_result = general_command(&mut db, &command, "key_px_arg value PX");
    assert_eq!(px_result, "Set Error: Invalid expired time".to_string());

    // test expired time ex and exat simultaneously
    let ex_exat_result = general_command(&mut db, &command, "key_ex_exat_arg value EX 60 EXAT 1000");
    assert_eq!(ex_exat_result, "Set Error: Invalid expired time in set".to_string());

    // test expired time px and pxat simultaneously
    let px_pxat_result = general_command(&mut db, &command, "key_px_pxat_arg value PX 60000 PXAT 1000");
    assert_eq!(px_pxat_result, "Set Error: Invalid expired time in set".to_string());

    // test expired time exat
    let exat_arg_result = general_command(&mut db, &command, format!("key_exat_arg value EXAT {}", get_current_time() / 1000 + 60).as_str());
    assert_eq!(exat_arg_result, "OK".to_string());
    let ttl_key_exat_result = ttl_command(&mut db, "ttl", "key_exat_arg");
    assert!(0 < ttl_key_exat_result && ttl_key_exat_result <= 60);

    // test expired time pxat
    let pxat_arg_result = general_command(&mut db, &command, format!("key_pxat_arg value PXAT {}", get_current_time() + 60000).as_str());
    assert_eq!(pxat_arg_result, "OK".to_string());
    let ttl_key_pxat_result = ttl_command(&mut db, "ttl", "key_pxat_arg");
    assert!(0 < ttl_key_pxat_result && ttl_key_pxat_result <= 60);

    // test nx and xx simultaneously
    let nx_xx_result = general_command(&mut db, &command, "key_nx_xx_arg value NX XX");
    assert_eq!(nx_xx_result, "Set Error: nx and xx cannot exist simultaneously".to_string());

    // test nx arg
    let nx_result = general_command(&mut db, &command, "key_nx_arg value NX");
    assert_eq!(nx_result, "OK".to_string());
    assert(&mut db, "key_nx_arg", "value");

    // test xx arg key not exist
    let xx_result = general_command(&mut db, &command, "key_xx_not_arg value XX");
    assert_eq!(xx_result, "Set Error: Key does not exist".to_string());

    // test xx arg key exist
    let result = general_command(&mut db, &command, "key_xx_arg value");
    assert_eq!(result, "OK".to_string());
    assert(&mut db, "key_xx_arg", "value");
    let xx_result = general_command(&mut db, &command, "key_xx_arg value XX");
    assert_eq!(xx_result, "OK".to_string());
    assert(&mut db, "key_xx_arg", "value");
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