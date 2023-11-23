use std::error::Error;
use rus_key::db::Db;
use rus_key::db::DataType;
use rus_key::func::string::StringCommand;
use rus_key::func::expired::ExpiredCommand;
use rus_key::func::utils::UtilsCommand;
use rus_key::command_factory::Command;

fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp_millis();
    timestamp
}

fn ttl_command(db: &mut Db, command: &str, key: &str) -> Result<i64, Box<dyn Error>> {
    let command_ttl = ExpiredCommand::new(command.to_string());
    let command_ttl_str = key;
    let mut parts_ttl = command_ttl_str.split_ascii_whitespace();
    let result = command_ttl.execute(&mut parts_ttl, db);
    match result {
        Ok(value) => Ok(value.parse::<i64>()?),
        Err(e) => Err(e.into()),
    }
}

fn general_command(db: &mut Db, command_set: &StringCommand, command_set_str: &str) -> Result<String, Box<dyn Error>> {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    match result_set {
        Ok(value) => Ok(value),
        Err(e) => Err(e.into()),
    }
}

fn assert_command(db: &mut Db, command_set: &StringCommand, args: &str, key: &str, expected_result: &str, expected_value: &str, is_ttl: Option<bool>, ttl: Option<i64>) -> Result<(), Box<dyn Error>> {
    let general_result = general_command(db, command_set, args)?;
    assert_eq!(general_result, expected_result);
    if !expected_value.is_empty() {
        assert_eq!(
            match db.get(key) {
                Some(DataType::String(actual_value)) => actual_value,
                _ => panic!("Key not found"),
            },
            expected_value
        ); 
    }

    if is_ttl.is_some() {
        let ttl_result = ttl_command(db, "ttl", key)?;
        if ttl.unwrap() == -1 {
            assert_eq!(ttl_result, -1);
        } else {
            assert!(0 < ttl_result && ttl_result <= ttl.unwrap());
        }
    }

    Ok(())
}

#[test]
fn test_append_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let exists_command = UtilsCommand::new("exists".to_string());
    let append_command = StringCommand::new("append".to_string());
    let getrange_command = StringCommand::new("getrange".to_string());
    let get_command = StringCommand::new("get".to_string());

    // ensure that the key does not exist
    let command_exists_str = "key";
    let mut parts_exists = command_exists_str.split_ascii_whitespace();
    let result_exists = exists_command.execute(&mut parts_exists, &mut db)?;
    assert_eq!(result_exists, "0".to_string());

    let tests_case: Vec<(&str, &str, &str, &StringCommand)> = vec![
        ("key", "key", "nil", &get_command),
        ("key value_1", "key", "7", &append_command),
        ("key value_2", "key", "14", &append_command),
        ("key", "key", "value_1value_2", &get_command),
        ("key value_3", "key", "21", &append_command),
        ("key", "key", "value_1value_2value_3", &get_command),
        ("ts 0043", "ts", "4", &append_command),
        ("ts 0035", "ts", "8", &append_command),
        ("ts 0 3", "ts", "0043", &getrange_command),
        ("ts 4 7", "ts", "0035", &getrange_command),
    ];

    for (args, key, expected_result, command) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}", args, key, expected_result);
        assert_command(&mut db, command, args, key, expected_result, "", None, None)?;
    }

    Ok(())
}

#[test]
fn test_decr_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let decr_command = StringCommand::new("decr".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key_not_decr", "key_not_decr", "-1", "-1", &decr_command),
        ("key_int 10", "key_int", "OK", "10", &set_command),
        ("key_int", "key_int", "9", "9", &decr_command),
        ("key_int 234293482390480948029348230948", "key_int", "OK", "234293482390480948029348230948", &set_command),
        ("key_int", "key_int", "Decr Error: Value is not an integer or out of range", "234293482390480948029348230948", &decr_command),
        ("key_int -9223372036854775808", "key_int", "OK", "-9223372036854775808", &set_command),
        ("key_int", "key_int", "Decr Error: Value is not an integer or out of range", "-9223372036854775808", &decr_command),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        ("key_not_int", "key_not_int", "Decr Error: Value is not an integer or out of range", "1.1", &decr_command),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        ("key_not_int", "key_not_int", "Decr Error: Value is not an integer or out of range", "abc", &decr_command),
        ("key_max 9223372036854775807", "key_max", "OK", "9223372036854775807", &set_command),
        ("key_max", "key_max", "9223372036854775806", "9223372036854775806", &decr_command),
        ("key_min_plus_one -9223372036854775807", "key_min_plus_one", "OK", "-9223372036854775807", &set_command),
        ("key_min_plus_one", "key_min_plus_one", "-9223372036854775808", "-9223372036854775808", &decr_command),
        ("key_repeat 5", "key_repeat", "OK", "5", &set_command),
        ("key_repeat", "key_repeat", "4", "4", &decr_command),
        ("key_repeat", "key_repeat", "3", "3", &decr_command),
        ("key_repeat", "key_repeat", "2", "2", &decr_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}, expected_value: {}", args, key, expected_result, expected_value);
        assert_command(&mut db, command, args, key, expected_result, expected_value, None, None)?;
    }

    Ok(())
}

#[test]
fn test_decrby_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();

    let decrby_command = StringCommand::new("decrby".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key_not_decrby", "key_not_decrby", "ERR wrong number of arguments for command", "", &decrby_command),
        ("key_not_decrby 1", "key_not_decrby", "-1", "-1", &decrby_command),
        ("key_int 10", "key_int", "OK", "10", &set_command),
        ("key_int 1", "key_int", "9", "9", &decrby_command),
        ("key_int 2", "key_int", "7", "7", &decrby_command),
        ("key_int -1", "key_int", "8", "8", &decrby_command),
        ("key_int 234293482390480948029348230948", "key_int", "OK", "234293482390480948029348230948", &set_command),
        ("key_int 2", "key_int", "Decr Error: Value is not an integer or out of range", "234293482390480948029348230948", &decrby_command),
        ("key_int -9223372036854775808", "key_int", "OK", "-9223372036854775808", &set_command),
        ("key_int -1", "key_int", "-9223372036854775807", "-9223372036854775807", &decrby_command),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        ("key_not_int 1", "key_not_int", "Decr Error: Value is not an integer or out of range", "1.1", &decrby_command),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        ("key_not_int 1", "key_not_int", "Decr Error: Value is not an integer or out of range", "abc", &decrby_command),
        ("key_max 9223372036854775807", "key_max", "OK", "9223372036854775807", &set_command),
        ("key_max 2", "key_max", "9223372036854775805", "9223372036854775805", &decrby_command),
        ("key_min -9223372036854775807", "key_min", "OK", "-9223372036854775807", &set_command),
        ("key_min 2", "key_min", "Decr Error: Value is not an integer or out of range", "-9223372036854775807", &decrby_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}, expected_value: {}", args, key, expected_result, expected_value);
        assert_command(&mut db, command, args, key, expected_result, expected_value, None, None)?;
    }

    Ok(())
}

#[test]
fn test_get_del_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let getdel_command = StringCommand::new("getdel".to_string());
    let get_command = StringCommand::new("get".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key value", "key", "OK", "value", &set_command),
        ("key", "key", "value", "", &get_command),
        ("key", "key", "value", "", &getdel_command),
        ("key", "key", "nil", "", &get_command),
        ("key_not_exist", "key_not_exist", "nil", "", &getdel_command),
        ("key_not_exist", "key_not_exist", "nil", "", &get_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}, expected_value: {}", args, key, expected_result, expected_value);
        assert_command(&mut db, command, args, key, expected_result, expected_value, None, None)?;
    }

    Ok(())
}

#[test]
fn test_set_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());

    let exat_string = format!("key_exat_arg value EXAT {}", get_current_time() / 1000 + 60);
    let pxat_string = format!("key_pxat_arg value PXAT {}", get_current_time() + 60000);

    let tests_case: Vec<(&str, &str, &str, &str, Option<bool>, Option<i64>)> = vec![
        ("key value", "key", "OK", "value", None, None), // test with a single parameter
        ("key value1 value2", "key", "OK", "value1", None, None), // test with multiple parameters
        ("key \"This is value\"", "key", "OK", "This is value", None, None), // test with a value containing spaces
        ("key_ex_arg value EX 60", "key_ex_arg", "OK", "value", Some(true), Some(60)), // extra parameter
        ("key_px_arg value PX 60000", "key_px_arg", "OK", "value", Some(true), Some(60)), // test expired time PX milliseconds
        (&exat_string, "key_exat_arg", "OK", "value", Some(true), Some(60)), // test expired time EXAT
        (&pxat_string, "key_pxat_arg", "OK", "value", Some(true), Some(60)), // test expired time PXAT
        ("key_ex_px_arg value EX 60 PX 60000", "key_ex_px_arg", "Set Error: Invalid expired time in set", "", None, None), // error test ex and px cannot exist simultaneously
        ("key_ex_exat_arg value EX 60 EXAT 1700360582694", "key_ex_exat_arg", "Set Error: Invalid expired time in set", "", None, None), // error test ex and exat cannot exist simultaneously
        ("key_px_pxat_arg value PX 60000 PXAT 1700360582694000", "key_px_pxat_arg", "Set Error: Invalid expired time in set", "", None, None), // error test px and pxat cannot exist simultaneously
        ("key_nx_xx_arg value NX XX", "key_nx_xx_arg", "Set Error: nx and xx cannot exist simultaneously", "", None, None), // error test nx and xx cannot exist simultaneously
        ("key_nx_arg value NX", "key_nx_arg", "OK", "value", None, None), // test nx arg
        ("key_xx_not_arg value XX", "key_xx_not_arg", "Set Error: Key does not exist", "", None, None), // test xx arg key not exist
        ("key_xx_arg value", "key_xx_arg", "OK", "value", None, None), // test xx arg key exist
        ("key_xx_arg value_exist XX", "key_xx_arg", "OK", "value_exist", None, None), // test xx arg key exist
        ("key_ex_arg value EX 60", "key_ex_arg", "OK", "value", Some(true), Some(60)), // test keepttl arg
        ("key_ex_arg value_keepttl KEEPTTL", "key_ex_arg", "OK", "value_keepttl", Some(true), Some(60)), // test keepttl arg
        ("key_ex_arg value_not_keepttl", "key_ex_arg", "OK", "value_not_keepttl", Some(true), Some(-1)), // test not keepttl arg
        ("key_get old_value", "key_get", "OK", "old_value", None, None), // test get arg set key_get
        ("key_get new_value get", "key_get", "old_value", "new_value", None, None), // test get arg
    ];

    for (args, key, expected_result, expected_value, is_ttl, ttl) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}, expected_value: {}, is_ttl: {:?}, ttl: {:?}", args, key, expected_result, expected_value, is_ttl, ttl);
        assert_command(&mut db, &command, args, key, expected_result, expected_value, is_ttl, ttl)?;
    }

    Ok(())
}

#[test]
fn test_get_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let set_result = general_command(&mut db, &set_command, "key value")?;
    assert_eq!(set_result, "OK".to_string());

    let get_command = StringCommand::new("get".to_string());
    let get_result = general_command(&mut db, &get_command, "key")?;
    assert_eq!(get_result, "value".to_string());

    Ok(())
}

#[test]
fn test_getrange_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let command = StringCommand::new("set".to_string());
    let result = general_command(&mut db, &command, "key \"This is a string\"")?;
    assert_eq!(result, "OK".to_string());
    assert_eq!(
        match db.get("key") {
            Some(DataType::String(value)) => value,
            _ => panic!("Key not found"),
        },
        "This is a string"
    );

    let command_getrange = StringCommand::new("getrange".to_string());
    let tests_case: Vec<(&str, &str, &str, &str)> = vec![
        ("key 0 3", "key", "This", ""),
        ("key 0 16", "key", "This is a string", ""),
        ("key 0 -1", "key", "This is a string", ""),
        ("key -3 -1", "key", "ing", ""),
        ("key 10 100", "key", "string", ""),
        ("key 0 0", "key", "T", ""),
        ("key 2 1", "key", "", ""),
        ("key -20 -1", "key", "This is a string", ""),
        ("key -20 -19", "key", "T", ""),
        ("non_existent_key 0 -2", "non_existent_key", "", ""),
    ];

    for (args, key, expected_result, expected_value) in tests_case {
        println!("arg: {}, key: {}, expected_result: {}, expected_value: {}", args, key, expected_result, expected_value);
        assert_command(&mut db, &command_getrange, args, key, expected_result, expected_value, None, None)?;
    }

    Ok(())
}