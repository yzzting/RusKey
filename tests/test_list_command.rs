use list_commands::list::ListCommand;
use rus_key_db::db::{DataType, Db};
use rus_key_trait::command_trait::Command;
use std::collections::VecDeque;
use std::error::Error;

fn generate_vec_deque(value: Vec<&str>) -> VecDeque<String> {
    let mut value_vec: VecDeque<String> = VecDeque::new();
    for v in value {
        value_vec.push_back(v.to_string());
    }
    value_vec
}

fn general_command(
    db: &mut Db,
    command_set: &ListCommand,
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
    command_set: &ListCommand,
    args: &str,
    key: &str,
    expected_result: &str,
    expected_value: VecDeque<String>,
) -> Result<(), Box<dyn Error>> {
    let general_result = general_command(db, command_set, args)?;
    let empty_vec_deque: VecDeque<String> = VecDeque::new();
    if !key.is_empty() {
        assert_eq!(general_result, expected_result);
    }
    let actual_value = match db.get(key) {
        Some(DataType::List(actual_value)) => actual_value,
        _ => &empty_vec_deque,
    };
    assert_eq!(*actual_value, expected_value);

    Ok(())
}

#[test]
fn test_list_push_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();

    let list_push_command = ListCommand::new("lpush".to_string());

    let tests_case: Vec<(&str, &str, &str, VecDeque<String>, &ListCommand)> = vec![
        (
            "key1 value1 value2",
            "key1",
            "2",
            generate_vec_deque(vec!["value1", "value2"]),
            &list_push_command,
        ),
        (
            "key1 value3 value4",
            "key1",
            "4",
            generate_vec_deque(vec!["value3", "value4", "value1", "value2"]),
            &list_push_command,
        ),
        (
            "key2 value1",
            "key2",
            "1",
            generate_vec_deque(vec!["value1"]),
            &list_push_command,
        ),
        (
            "key2 value2",
            "key2",
            "2",
            generate_vec_deque(vec!["value2", "value1"]),
            &list_push_command,
        ),
        (
            "key3 value1 value1",
            "key3",
            "2",
            generate_vec_deque(vec!["value1", "value1"]),
            &list_push_command,
        ),
        (
            "key4",
            "key4",
            "0",
            generate_vec_deque(vec![]),
            &list_push_command,
        ),
    ];
    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {:?}",
            args, key, expected_result, expected_value
        );
        assert_command(&mut db, command, args, key, expected_result, expected_value)?;
    }
    Ok(())
}

#[test]
fn test_list_push_x_command() -> Result<(), Box<dyn Error>> {
    let mut db = Db::new();

    let list_push_x_command = ListCommand::new("lpushx".to_string());
    let list_push_command = ListCommand::new("lpush".to_string());

    let tests_case: Vec<(&str, &str, &str, VecDeque<String>, &ListCommand)> = vec![
        (
            "key1 value1 value2",
            "key1",
            "0",
            generate_vec_deque(vec![]),
            &list_push_x_command,
        ),
        (
            "key1 value1 value2",
            "key1",
            "2",
            generate_vec_deque(vec!["value1", "value2"]),
            &list_push_command,
        ),
        (
            "key1 value3 value4",
            "key1",
            "4",
            generate_vec_deque(vec!["value3", "value4", "value1", "value2"]),
            &list_push_x_command,
        ),
        (
            "key2 value1",
            "key2",
            "0",
            generate_vec_deque(vec![]),
            &list_push_x_command,
        ),
        (
            "key2 value1",
            "key2",
            "1",
            generate_vec_deque(vec!["value1"]),
            &list_push_command,
        ),
        (
            "key2 value2 value3",
            "key2",
            "3",
            generate_vec_deque(vec!["value2", "value3", "value1"]),
            &list_push_x_command,
        ),
        (
            "key3 value1 value1",
            "key3",
            "0",
            generate_vec_deque(vec![]),
            &list_push_x_command,
        ),
        (
            "key3 value1 value1",
            "key3",
            "2",
            generate_vec_deque(vec!["value1", "value1"]),
            &list_push_command,
        ),
        (
            "key3 value2",
            "key3",
            "3",
            generate_vec_deque(vec!["value2", "value1", "value1"]),
            &list_push_x_command,
        ),
        (
            "key3 value2",
            "key3",
            "4",
            generate_vec_deque(vec!["value2", "value2", "value1", "value1"]),
            &list_push_x_command,
        ),
    ];
    for (args, key, expected_result, expected_value, command) in tests_case {
        println!(
            "arg: {}, key: {}, expected_result: {}, expected_value: {:?}",
            args, key, expected_result, expected_value
        );
        assert_command(&mut db, command, args, key, expected_result, expected_value)?;
    }
    Ok(())
}
