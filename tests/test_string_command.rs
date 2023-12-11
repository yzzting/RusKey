use rus_key::command_factory::Command;
use rus_key::db::DataType;
use rus_key::db::Db;
use rus_key::commands::expired::ExpiredCommand;
use rus_key::commands::string::StringCommand;
use rus_key::commands::utils::UtilsCommand;
use std::error::Error;

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

fn general_command(
    db: &mut Db,
    command_set: &StringCommand,
    command_set_str: &str,
) -> Result<String, Box<dyn Error>> {
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    match result_set {
        Ok(value) => Ok(value),
        Err(e) => Err(e.into()),
    }
}

fn assert_command(
    db: &mut Db,
    command_set: &StringCommand,
    args: &str,
    key: &str,
    expected_result: &str,
    expected_value: &str,
    is_ttl: Option<bool>,
    ttl: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let general_result = general_command(db, command_set, args)?;
    if !key.is_empty() {
        assert_eq!(general_result, expected_result);
    }
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
        println!(
            "arg: {}, key: {}, expected_result: {}",
            args, key, expected_result
        );
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
        (
            "key_int 234293482390480948029348230948",
            "key_int",
            "OK",
            "234293482390480948029348230948",
            &set_command,
        ),
        (
            "key_int",
            "key_int",
            "Value is not an integer or out of range",
            "234293482390480948029348230948",
            &decr_command,
        ),
        (
            "key_int -9223372036854775808",
            "key_int",
            "OK",
            "-9223372036854775808",
            &set_command,
        ),
        (
            "key_int",
            "key_int",
            "Value is not an integer or out of range",
            "-9223372036854775808",
            &decr_command,
        ),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        (
            "key_not_int",
            "key_not_int",
            "Value is not an integer or out of range",
            "1.1",
            &decr_command,
        ),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        (
            "key_not_int",
            "key_not_int",
            "Value is not an integer or out of range",
            "abc",
            &decr_command,
        ),
        (
            "key_max 9223372036854775807",
            "key_max",
            "OK",
            "9223372036854775807",
            &set_command,
        ),
        (
            "key_max",
            "key_max",
            "9223372036854775806",
            "9223372036854775806",
            &decr_command,
        ),
        (
            "key_min_plus_one -9223372036854775807",
            "key_min_plus_one",
            "OK",
            "-9223372036854775807",
            &set_command,
        ),
        (
            "key_min_plus_one",
            "key_min_plus_one",
            "-9223372036854775808",
            "-9223372036854775808",
            &decr_command,
        ),
        ("key_repeat 5", "key_repeat", "OK", "5", &set_command),
        ("key_repeat", "key_repeat", "4", "4", &decr_command),
        ("key_repeat", "key_repeat", "3", "3", &decr_command),
        ("key_repeat", "key_repeat", "2", "2", &decr_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_decrby_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();

    let decrby_command = StringCommand::new("decrby".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key_not_decrby",
            "key_not_decrby",
            "ERR wrong number of arguments for command",
            "",
            &decrby_command,
        ),
        (
            "key_not_decrby 1",
            "key_not_decrby",
            "-1",
            "-1",
            &decrby_command,
        ),
        ("key_int 10", "key_int", "OK", "10", &set_command),
        ("key_int 1", "key_int", "9", "9", &decrby_command),
        ("key_int 2", "key_int", "7", "7", &decrby_command),
        ("key_int -1", "key_int", "8", "8", &decrby_command),
        (
            "key_int 234293482390480948029348230948",
            "key_int",
            "OK",
            "234293482390480948029348230948",
            &set_command,
        ),
        (
            "key_int 2",
            "key_int",
            "Value is not an integer or out of range",
            "234293482390480948029348230948",
            &decrby_command,
        ),
        (
            "key_int -9223372036854775808",
            "key_int",
            "OK",
            "-9223372036854775808",
            &set_command,
        ),
        (
            "key_int -1",
            "key_int",
            "-9223372036854775807",
            "-9223372036854775807",
            &decrby_command,
        ),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        (
            "key_not_int 1",
            "key_not_int",
            "Value is not an integer or out of range",
            "1.1",
            &decrby_command,
        ),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        (
            "key_not_int 1",
            "key_not_int",
            "Value is not an integer or out of range",
            "abc",
            &decrby_command,
        ),
        (
            "key_max 9223372036854775807",
            "key_max",
            "OK",
            "9223372036854775807",
            &set_command,
        ),
        (
            "key_max 2",
            "key_max",
            "9223372036854775805",
            "9223372036854775805",
            &decrby_command,
        ),
        (
            "key_min -9223372036854775807",
            "key_min",
            "OK",
            "-9223372036854775807",
            &set_command,
        ),
        (
            "key_min 2",
            "key_min",
            "Value is not an integer or out of range",
            "-9223372036854775807",
            &decrby_command,
        ),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
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
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_getex_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let getex_command = StringCommand::new("getex".to_string());

    let exat_string = format!("key value EXAT {}", get_current_time() / 1000 + 60);
    let pxat_string = format!("key value PXAT {}", get_current_time() + 60000);

    let tests_case: Vec<(
        &str,
        &str,
        &str,
        &str,
        &StringCommand,
        Option<bool>,
        Option<i64>,
    )> = vec![
        ("key value", "key", "OK", "value", &set_command, None, None),
        (
            "key",
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(-1),
        ),
        (
            "key EX 60",
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(60),
        ),
        (
            "key PX 60000",
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(60),
        ),
        (
            &exat_string,
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(60),
        ),
        (
            &pxat_string,
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(60),
        ),
        (
            "key value EX 60000 EXAT 1700360582694",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value PX 60000 PXAT 1700360582694000",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value PX 60000 EX 60",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value PXAT 1700360582694000 EXAT 1700360582694",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value EX 60000 PERSIST",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value PX 60000 PERSIST",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value EXAT 1700360582694 PERSIST",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key value PXAT 1700360582694000 PERSIST",
            "key",
            "Set Error: Invalid expired time in set",
            "",
            &getex_command,
            None,
            None,
        ),
        (
            "key EX 60",
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(60),
        ),
        (
            "key PERSIST",
            "key",
            "value",
            "",
            &getex_command,
            Some(true),
            Some(-1),
        ),
    ];

    for (args, key, expected_result, expected_value, command, is_ttl, ttl) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            is_ttl,
            ttl,
        )?;
    }

    Ok(())
}

#[test]
fn test_get_set_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let getset_command = StringCommand::new("getset".to_string());
    let get_command = StringCommand::new("get".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key value", "key", "OK", "value", &set_command),
        (
            "key new_value",
            "key",
            "value",
            "new_value",
            &getset_command,
        ),
        ("key", "key", "new_value", "", &get_command),
        (
            "key new_value_1",
            "key",
            "new_value",
            "new_value_1",
            &getset_command,
        ),
        ("key", "key", "new_value_1", "", &get_command),
        (
            "key_not_exist value",
            "key_not_exist",
            "nil",
            "",
            &getset_command,
        ),
        ("key_not_exist", "key_not_exist", "value", "", &get_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_incr_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let incr_command = StringCommand::new("incr".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key_not_incr", "key_not_incr", "1", "1", &incr_command),
        ("key_int 10", "key_int", "OK", "10", &set_command),
        ("key_int", "key_int", "11", "11", &incr_command),
        (
            "key_int 234293482390480948029348230948",
            "key_int",
            "OK",
            "234293482390480948029348230948",
            &set_command,
        ),
        (
            "key_int",
            "key_int",
            "Value is not an integer or out of range",
            "234293482390480948029348230948",
            &incr_command,
        ),
        (
            "key_int -9223372036854775808",
            "key_int",
            "OK",
            "-9223372036854775808",
            &set_command,
        ),
        (
            "key_int",
            "key_int",
            "-9223372036854775807",
            "-9223372036854775807",
            &incr_command,
        ),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        (
            "key_not_int",
            "key_not_int",
            "Value is not an integer or out of range",
            "1.1",
            &incr_command,
        ),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        (
            "key_not_int",
            "key_not_int",
            "Value is not an integer or out of range",
            "abc",
            &incr_command,
        ),
        (
            "key_max 9223372036854775807",
            "key_max",
            "OK",
            "9223372036854775807",
            &set_command,
        ),
        (
            "key_max",
            "key_max",
            "Value is not an integer or out of range",
            "9223372036854775807",
            &incr_command,
        ),
        (
            "key_min_plus_one -9223372036854775807",
            "key_min_plus_one",
            "OK",
            "-9223372036854775807",
            &set_command,
        ),
        (
            "key_min_plus_one",
            "key_min_plus_one",
            "-9223372036854775806",
            "-9223372036854775806",
            &incr_command,
        ),
        ("key_repeat 5", "key_repeat", "OK", "5", &set_command),
        ("key_repeat", "key_repeat", "6", "6", &incr_command),
        ("key_repeat", "key_repeat", "7", "7", &incr_command),
        ("key_repeat", "key_repeat", "8", "8", &incr_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_incrby_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let incrby_command = StringCommand::new("incrby".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key_not_incrby",
            "key_not_incrby",
            "ERR wrong number of arguments for command",
            "",
            &incrby_command,
        ),
        (
            "key_not_incrby 1",
            "key_not_incrby",
            "1",
            "1",
            &incrby_command,
        ),
        ("key_int 10", "key_int", "OK", "10", &set_command),
        ("key_int 1", "key_int", "11", "11", &incrby_command),
        ("key_int 2", "key_int", "13", "13", &incrby_command),
        ("key_int -1", "key_int", "12", "12", &incrby_command),
        (
            "key_int 234293482390480948029348230948",
            "key_int",
            "OK",
            "234293482390480948029348230948",
            &set_command,
        ),
        (
            "key_int 2",
            "key_int",
            "Value is not an integer or out of range",
            "234293482390480948029348230948",
            &incrby_command,
        ),
        (
            "key_int -9223372036854775808",
            "key_int",
            "OK",
            "-9223372036854775808",
            &set_command,
        ),
        (
            "key_int -1",
            "key_int",
            "Value is not an integer or out of range",
            "-9223372036854775808",
            &incrby_command,
        ),
        ("key_not_int 1.1", "key_not_int", "OK", "1.1", &set_command),
        (
            "key_not_int 1",
            "key_not_int",
            "Value is not an integer or out of range",
            "1.1",
            &incrby_command,
        ),
        ("key_not_int abc", "key_not_int", "OK", "abc", &set_command),
        (
            "key_not_int 1",
            "key_not_int",
            "Value is not an integer or out of range",
            "abc",
            &incrby_command,
        ),
        (
            "key_max 9223372036854775807",
            "key_max",
            "OK",
            "9223372036854775807",
            &set_command,
        ),
        (
            "key_max 2",
            "key_max",
            "Value is not an integer or out of range",
            "9223372036854775807",
            &incrby_command,
        ),
        (
            "key_min -9223372036854775807",
            "key_min",
            "OK",
            "-9223372036854775807",
            &set_command,
        ),
        (
            "key_min 2",
            "key_min",
            "-9223372036854775805",
            "-9223372036854775805",
            &incrby_command,
        ),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_incrbyfloat_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let incrbyfloat_command = StringCommand::new("incrbyfloat".to_string());
    let set_command = StringCommand::new("set".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key_not_incrbyfloat",
            "key_not_incrbyfloat",
            "ERR wrong number of arguments for command",
            "",
            &incrbyfloat_command,
        ),
        (
            "key_not_incrbyfloat 1.1",
            "key_not_incrbyfloat",
            "1.1",
            "1.1",
            &incrbyfloat_command,
        ),
        ("key_float 10.1", "key_float", "OK", "10.1", &set_command),
        (
            "key_float 1.1",
            "key_float",
            "11.2",
            "11.2",
            &incrbyfloat_command,
        ),
        (
            "key_float 2.1",
            "key_float",
            "13.3",
            "13.3",
            &incrbyfloat_command,
        ),
        (
            "key_float -1.1",
            "key_float",
            "12.2",
            "12.2",
            &incrbyfloat_command,
        ),
        // (
        //     "key_float 1.0E308",
        //     "key_float",
        //     "OK",
        //     "1.0E308",
        //     &set_command,
        // ),
        // (
        //     "key_float 0.7E308",
        //     "key_float",
        //     "1.7E308",
        //     "1.7E308",
        //     &incrbyfloat_command,
        // ),
        // (
        //     "key_float -1.7E308",
        //     "key_float",
        //     "OK",
        //     "-1.7E308",
        //     &set_command,
        // ),
        // (
        //     "key_float 1.1",
        //     "key_float",
        //     "Value is not an integer or out of range",
        //     "-1.7E308",
        //     &incrbyfloat_command,
        // ),
        // (
        //     "key_float -1.7E308",
        //     "key_float",
        //     "OK",
        //     "-1.7E308",
        //     &set_command,
        // ),
        // (
        //     "key_float -0.1",
        //     "key_float",
        //     "Value is not an integer or out of range",
        //     "-1.7E308",
        //     &incrbyfloat_command,
        // ),
        ("key_not_float 1", "key_not_float", "OK", "1", &set_command),
        (
            "key_not_float abc",
            "key_not_float",
            "Value is not an float or out of range",
            "1",
            &incrbyfloat_command,
        ),
        (
            "key_not_float abc",
            "key_not_float",
            "OK",
            "abc",
            &set_command,
        ),
        (
            "key_not_float 1.1",
            "key_not_float",
            "Value is not an float or out of range",
            "abc",
            &incrbyfloat_command,
        ),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
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
        (
            "\"key1 key2\" value",
            "key1 key2",
            "OK",
            "value",
            None,
            None,
        ), // test with a key containing spaces
        (
            "key \"This is value\"",
            "key",
            "OK",
            "This is value",
            None,
            None,
        ), // test with a value containing spaces
        (
            "key_ex_arg value EX 60",
            "key_ex_arg",
            "OK",
            "value",
            Some(true),
            Some(60),
        ), // extra parameter
        (
            "key_px_arg value PX 60000",
            "key_px_arg",
            "OK",
            "value",
            Some(true),
            Some(60),
        ), // test expired time PX milliseconds
        (
            &exat_string,
            "key_exat_arg",
            "OK",
            "value",
            Some(true),
            Some(60),
        ), // test expired time EXAT
        (
            &pxat_string,
            "key_pxat_arg",
            "OK",
            "value",
            Some(true),
            Some(60),
        ), // test expired time PXAT
        (
            "key_ex_px_arg value EX 60 PX 60000",
            "key_ex_px_arg",
            "Set Error: Invalid expired time in set",
            "",
            None,
            None,
        ), // error test ex and px cannot exist simultaneously
        (
            "key_ex_exat_arg value EX 60 EXAT 1700360582694",
            "key_ex_exat_arg",
            "Set Error: Invalid expired time in set",
            "",
            None,
            None,
        ), // error test ex and exat cannot exist simultaneously
        (
            "key_px_pxat_arg value PX 60000 PXAT 1700360582694000",
            "key_px_pxat_arg",
            "Set Error: Invalid expired time in set",
            "",
            None,
            None,
        ), // error test px and pxat cannot exist simultaneously
        (
            "key_nx_xx_arg value NX XX",
            "key_nx_xx_arg",
            "Set Error: nx and xx cannot exist simultaneously",
            "",
            None,
            None,
        ), // error test nx and xx cannot exist simultaneously
        (
            "key_nx_arg value NX",
            "key_nx_arg",
            "OK",
            "value",
            None,
            None,
        ), // test nx arg
        (
            "key_xx_not_arg value XX",
            "key_xx_not_arg",
            "Set Error: Key does not exist",
            "",
            None,
            None,
        ), // test xx arg key not exist
        ("key_xx_arg value", "key_xx_arg", "OK", "value", None, None), // test xx arg key exist
        (
            "key_xx_arg value_exist XX",
            "key_xx_arg",
            "OK",
            "value_exist",
            None,
            None,
        ), // test xx arg key exist
        (
            "key_ex_arg value EX 60",
            "key_ex_arg",
            "OK",
            "value",
            Some(true),
            Some(60),
        ), // test keepttl arg
        (
            "key_ex_arg value_keepttl KEEPTTL",
            "key_ex_arg",
            "OK",
            "value_keepttl",
            Some(true),
            Some(60),
        ), // test keepttl arg
        (
            "key_ex_arg value_not_keepttl",
            "key_ex_arg",
            "OK",
            "value_not_keepttl",
            Some(true),
            Some(-1),
        ), // test not keepttl arg
        (
            "key_get old_value",
            "key_get",
            "OK",
            "old_value",
            None,
            None,
        ), // test get arg set key_get
        (
            "key_get new_value get",
            "key_get",
            "old_value",
            "new_value",
            None,
            None,
        ), // test get arg
    ];

    for (args, key, expected_result, expected_value, is_ttl, ttl) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}, is_ttl: {:?}, ttl: {:?}",
            args, key, expected_result, expected_value, is_ttl, ttl
        );
        assert_command(
            &mut db,
            &command,
            args,
            key,
            expected_result,
            expected_value,
            is_ttl,
            ttl,
        )?;
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
    let set_command = StringCommand::new("set".to_string());
    // let result = general_command(&mut db, &command, "key \"This is a string\"")?;
    // assert_eq!(result, "OK".to_string());
    // assert_eq!(
    //     match db.get("key") {
    //         Some(DataType::String(value)) => value,
    //         _ => panic!("Key not found"),
    //     },
    //     "This is a string"
    // );

    let getrange_command = StringCommand::new("getrange".to_string());
    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key \"This is a string\"",
            "key",
            "OK",
            "This is a string",
            &set_command,
        ),
        ("key 0 3", "key", "This", "", &getrange_command),
        ("key 0 16", "key", "This is a string", "", &getrange_command),
        ("key 0 -1", "key", "", "", &getrange_command),
        ("key -3 -1", "key", "ing", "", &getrange_command),
        ("key 10 100", "key", "string", "", &getrange_command),
        ("key 0 0", "key", "T", "", &getrange_command),
        ("key 2 1", "key", "", "", &getrange_command),
        (
            "key -20 -1",
            "key",
            "This is a string",
            "",
            &getrange_command,
        ),
        ("key -20 -19", "key", "", "", &getrange_command),
        (
            "non_existent_key 0 -2",
            "non_existent_key",
            "",
            "",
            &getrange_command,
        ),
        ("single_char S", "single_char", "OK", "S", &set_command),
        ("single_char 0 0", "single_char", "S", "", &getrange_command),
        ("empty_str \"\"", "empty_str", "OK", "", &set_command),
        ("empty_str 0 0", "empty_str", "", "", &getrange_command),
        (
            "special_chars #$%^&",
            "special_chars",
            "OK",
            "#$%^&",
            &set_command,
        ),
        (
            "special_chars 0 5",
            "special_chars",
            "#$%^&",
            "",
            &getrange_command,
        ),
        (
            "non_ascii 你好世界",
            "non_ascii",
            "OK",
            "你好世界",
            &set_command,
        ),
        (
            "non_ascii 0 5",
            "non_ascii",
            "你好世界",
            "",
            &getrange_command,
        ),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_strlen_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let strlen_command = StringCommand::new("strlen".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key \"This is a string\"",
            "key",
            "OK",
            "This is a string",
            &set_command,
        ),
        ("key", "key", "16", "", &strlen_command),
        ("key_not_exist", "key_not_exist", "0", "", &strlen_command),
        ("key_num 10", "key_num", "OK", "10", &set_command),
        ("key_num", "key_num", "2", "", &strlen_command),
        ("key_num 100", "key_num", "OK", "100", &set_command),
        ("key_num", "key_num", "3", "", &strlen_command),
        ("key_empty \"\"", "key_empty", "OK", "", &set_command),
        ("key_empty", "key_empty", "0", "", &strlen_command),
        (
            "key_unicode \"你好\"",
            "key_unicode",
            "OK",
            "你好",
            &set_command,
        ),
        ("key_unicode", "key_unicode", "6", "", &strlen_command),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_setrange_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let set_command = StringCommand::new("set".to_string());
    let setrange_command = StringCommand::new("setrange".to_string());
    let get_command = StringCommand::new("get".to_string());

    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key \"Hello World\"",
            "key",
            "OK",
            "Hello World",
            &set_command,
        ),
        ("key 6 Redis", "key", "11", "Hello Redis", &setrange_command),
        ("key", "key", "Hello Redis", "", &get_command),
        ("key_nil 6 Redis", "key", "11", "", &setrange_command),
        // Set empty string
        ("key_empty \"\"", "key_empty", "OK", "", &set_command),
        (
            "key_empty 6 Redis",
            "key_empty",
            "11",
            "      Redis",
            &setrange_command,
        ),
        ("key_empty", "key_empty", "      Redis", "", &get_command),
        // Set num greater than string length
        (
            "key_num \"Hello World\"",
            "key_num",
            "OK",
            "Hello World",
            &set_command,
        ),
        (
            "key_num 13 Redis",
            "key_num",
            "18",
            "Hello World  Redis",
            &setrange_command,
        ),
        ("key_num", "key_num", "Hello World  Redis", "", &get_command),
        // Set short string
        (
            "key_short \"Hello\"",
            "key_short",
            "OK",
            "Hello",
            &set_command,
        ),
        (
            "key_short 6 Redis",
            "key_short",
            "11",
            "Hello Redis",
            &setrange_command,
        ),
        ("key_short", "key_short", "Hello Redis", "", &get_command),
        // Check lack arg
        (
            "key_lack_arg",
            "key_lack_arg",
            "ERR wrong number of arguments for command",
            "",
            &setrange_command,
        ),
        (
            "key_lack_arg 6",
            "key_lack_arg",
            "ERR wrong number of arguments for command",
            "",
            &setrange_command,
        ),
        (
            "key_lack_arg -1 Redis",
            "key_lack_arg",
            "ERR wrong number of arguments for command",
            "",
            &setrange_command,
        ),
    ];

    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }

    Ok(())
}

#[test]
fn test_mset_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();
    let mset_command = StringCommand::new("mset".to_string());
    let get_command = StringCommand::new("get".to_string());
    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        (
            "key1 value1 key2 value2",
            "key1",
            "OK",
            "value1",
            &mset_command,
        ),
        ("key1", "key1", "value1", "", &get_command),
        ("key2", "key2", "value2", "", &get_command),
        (
            "key1 value1 key2 value2 key3 value3",
            "key1",
            "OK",
            "value1",
            &mset_command,
        ),
        ("key1", "key1", "value1", "", &get_command),
        ("key2", "key2", "value2", "", &get_command),
        ("key3", "key3", "value3", "", &get_command),
        (
            "key1 \"Hello value1\" key2 \"Hello value2\" key3 \"Hello value3\"",
            "key1",
            "OK",
            "Hello value1",
            &mset_command,
        ),
        (
            "key1",
            "key1",
            "wrong number of arguments for 'mset' command",
            "",
            &mset_command,
        ),
        (
            "key1 value1 key2",
            "key1",
            "wrong number of arguments for 'mset' command",
            "",
            &mset_command,
        ),
        // Test with empty string as key/value
        ("key1 \"\" key2 value2", "key1", "OK", "", &mset_command),
        ("key1", "key1", "", "", &get_command),
        // Test with special characters in key/value
        (
            "key1 !@#$%^&*() key2 value2",
            "key1",
            "OK",
            "!@#$%^&*()",
            &mset_command,
        ),
        ("key1", "key1", "!@#$%^&*()", "", &get_command),
        // Test with unicode characters in key/value
        (
            "key1 \"你好\" key2 value2",
            "key1",
            "OK",
            "你好",
            &mset_command,
        ),
        (
            "key1",
            "key1",
            "你好",
            "",
            &get_command,
        ),
        // Test with a large number of key/value pairs
        (
            "key1 value1 key2 value2 key3 value3 key4 value4 key5 value5 key6 value6 key7 value7 key8 value8 key9 value9 key10 value10",
            "key1",
            "OK",
            "value1",
            &mset_command,
        ),
        (
            "key10",
            "key10",
            "value10",
            "",
            &get_command,
        ),
        // Test with keys that have whitespaces
        (
            "\"key 1\" value1 \"key 2\" value2",
            "key 1",
            "OK",
            "value1",
            &mset_command,
        ),
        (
            "\"key 1\"",
            "key 1",
            "value1",
            "",
            &get_command,
        ),
        (
            "\"key 2\"",
            "key 2",
            "value2",
            "",
            &get_command,
        ),
    ];
    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }
    Ok(())
}

#[test]
fn test_mget_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();

    let mset_command = StringCommand::new("mset".to_string());
    let mget_command = StringCommand::new("mget".to_string());
    let tests_case: Vec<(&str, &str, &str, &str, &StringCommand)> = vec![
        ("key1 value1 key2 value2", "key1 key2", "OK", "", &mset_command),
        ("key1 key2", "key1 key2", "value1 value2", "", &mget_command),
        (
            "key1 value1 key2 value2 key3 value3",
            "key1 key2 key3",
            "OK",
            "",
            &mset_command,
        ),
        ("key1 key2 key3", "key1 key2 key3", "value1 value2 value3", "", &mget_command),
        (
            "key1 \"Hello value1\" key2 \"Hello value2\" key3 \"Hello value3\"",
            "key1 key2 key3",
            "OK",
            "",
            &mset_command,
        ),
        (
            "key1 key2 key3",
            "key1 key2 key3",
            "Hello value1 Hello value2 Hello value3",
            "",
            &mget_command,
        ),
        (
            "key1 key2_not_exist key3_not_exist",
            "key1_not_exist key2_not_exist key3_not_exist",
            "Hello value1 nil nil",
            "",
            &mget_command,
        ),
        (
            "key1_not_exist key2_not_exist key3_not_exist",
            "key1_not_exist key2_not_exist key3_not_exist",
            "nil nil nil",
            "",
            &mget_command,
        ),
    ];
    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {}",
            args, key, expected_result, expected_value
        );
        assert_command(
            &mut db,
            command,
            args,
            key,
            expected_result,
            expected_value,
            None,
            None,
        )?;
    }
    Ok(())
}
