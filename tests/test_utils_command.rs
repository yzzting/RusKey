use rus_key_trait::command_trait::Command;
use rus_key_lib::db::{DataType, Db};
use hashmap_commands::hashmap::HashMapCommand;
use string_commands::string::StringCommand;
use utils_commands::utils::UtilsCommand;

fn set_key(db: &mut Db) {
    // set key
    let command_set = StringCommand::new("set".to_string());
    let command_set_str = "key value";
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    assert_eq!(result_set.unwrap(), "OK".to_string());

    // set other key
    let command_set = StringCommand::new("set".to_string());
    let command_set_str = "other_key value";
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    assert_eq!(result_set.unwrap(), "OK".to_string());

    // set hash
    let command_hmset = HashMapCommand::new("hmset".to_string());
    let command_hmset_str = "hash_key field1 value1 field2 value2";
    let mut parts_hmset = command_hmset_str.split_ascii_whitespace();
    let result_hmset = command_hmset.execute(&mut parts_hmset, db);
    assert_eq!(result_hmset.unwrap(), "OK".to_string());
}

fn random_generate_key(db: &mut Db) -> Vec<String> {
    let command_set = StringCommand::new("set".to_string());
    let mut key_arr = Vec::with_capacity(100);
    for i in 0..100 {
        let command_set_str = format!("key_{} value", i);
        let mut parts_set = command_set_str.split_ascii_whitespace();
        let result_set = command_set.execute(&mut parts_set, db);
        assert_eq!(result_set.unwrap(), "OK".to_string());
        key_arr.push(format!("key_{}", i))
    }
    key_arr
}

#[test]
fn test_rename_command() {
    let mut db = Db::new();
    set_key(&mut db);

    let command_rename = UtilsCommand::new("rename".to_string());
    // rename key not exists
    let command_rename_str = "key_not_exists new_key";
    let mut parts_rename = command_rename_str.split_ascii_whitespace();
    let result = command_rename.execute(&mut parts_rename, &mut db);
    assert_eq!(result, Err("No such key"));

    let command_rename_str = "key new_key";
    let mut parts_rename = command_rename_str.split_ascii_whitespace();
    let result = command_rename.execute(&mut parts_rename, &mut db);
    assert_eq!(result.unwrap(), "1".to_string());

    // test old key exists
    let command_exists = UtilsCommand::new("exists".to_string());
    let command_exists_str = "key";
    let mut parts_exists = command_exists_str.split_ascii_whitespace();
    let result = command_exists.execute(&mut parts_exists, &mut db);
    assert_eq!(result.unwrap(), "0".to_string());

    // test new key exists
    let command_exists = UtilsCommand::new("exists".to_string());
    let command_exists_str = "new_key";
    let mut parts_exists = command_exists_str.split_ascii_whitespace();
    let result = command_exists.execute(&mut parts_exists, &mut db);
    assert_eq!(result.unwrap(), "1".to_string());

    // get new key
    let command_get = StringCommand::new("get".to_string());
    let command_get_str = "new_key";
    let mut parts_get = command_get_str.split_ascii_whitespace();
    let result = command_get.execute(&mut parts_get, &mut db);
    assert_eq!(result.unwrap(), "value".to_string());
}

#[test]
fn test_renamenx_command() {
    let mut db = Db::new();
    set_key(&mut db);

    let command_rename = UtilsCommand::new("renamenx".to_string());
    // rename other_key is exists
    let command_rename_str = "key other_key";
    let mut parts_rename = command_rename_str.split_ascii_whitespace();
    let result = command_rename.execute(&mut parts_rename, &mut db);
    assert_eq!(result, Err("New name is exists"));

    // rename key not exists
    let command_rename_str = "key_not_exists new_key";
    let mut parts_rename = command_rename_str.split_ascii_whitespace();
    let result = command_rename.execute(&mut parts_rename, &mut db);
    assert_eq!(result, Err("No such key"));

    // rename key
    let command_rename_str = "key new_key";
    let mut parts_rename = command_rename_str.split_ascii_whitespace();
    let result = command_rename.execute(&mut parts_rename, &mut db);
    assert_eq!(result.unwrap(), "1".to_string());

    // test old key exists
    let command_exists = UtilsCommand::new("exists".to_string());
    let command_exists_str = "key";
    let mut parts_exists = command_exists_str.split_ascii_whitespace();
    let result = command_exists.execute(&mut parts_exists, &mut db);
    assert_eq!(result.unwrap(), "0".to_string());

    // test new key exists
    let command_exists = UtilsCommand::new("exists".to_string());
    let command_exists_str = "new_key";
    let mut parts_exists = command_exists_str.split_ascii_whitespace();
    let result = command_exists.execute(&mut parts_exists, &mut db);
    assert_eq!(result.unwrap(), "1".to_string());

    // get new key
    let command_get = StringCommand::new("get".to_string());
    let command_get_str = "new_key";
    let mut parts_get = command_get_str.split_ascii_whitespace();
    let result = command_get.execute(&mut parts_get, &mut db);
    assert_eq!(result.unwrap(), "value".to_string());
}

#[test]
fn test_randomkey_command() {
    let mut db = Db::new();
    // test randomkey when db is empty
    let command_randomkey = UtilsCommand::new("randomkey".to_string());
    let mut parts_randomkey = "".split_ascii_whitespace();
    let result = command_randomkey.execute(&mut parts_randomkey, &mut db);
    assert_eq!(result.unwrap(), "nil".to_string());

    let key_arr = random_generate_key(&mut db);

    let command_randomkey = UtilsCommand::new("randomkey".to_string());
    let mut parts_randomkey = "".split_ascii_whitespace();
    let result = command_randomkey.execute(&mut parts_randomkey, &mut db);
    let result_str = match result {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };
    assert!(key_arr.contains(&result_str));
}

#[test]
fn test_del_command() {
    let mut db = Db::new();
    // test del when db is empty
    let command_del = UtilsCommand::new("del".to_string());
    let command_del_str = "key";
    let mut parts_del = command_del_str.split_ascii_whitespace();
    let result = command_del.execute(&mut parts_del, &mut db);
    assert_eq!(result.unwrap(), "0".to_string());

    // test del key is exists
    set_key(&mut db);
    let command_del = UtilsCommand::new("del".to_string());
    let command_del_str = "key";
    let mut parts_del = command_del_str.split_ascii_whitespace();
    let result = command_del.execute(&mut parts_del, &mut db);
    assert_eq!(result.unwrap(), "1".to_string());
}

#[test]
fn test_type_command() {
    let mut db = Db::new();
    set_key(&mut db);

    // string
    let command_type = UtilsCommand::new("type".to_string());
    let command_type_str = "key";
    let mut parts_type = command_type_str.split_ascii_whitespace();
    let result = command_type.execute(&mut parts_type, &mut db);
    assert_eq!(result.unwrap(), "string".to_string());

    // hash
    let command_type = UtilsCommand::new("type".to_string());
    let command_type_str = "hash_key";
    let mut parts_type = command_type_str.split_ascii_whitespace();
    let result = command_type.execute(&mut parts_type, &mut db);
    assert_eq!(result.unwrap(), "zset".to_string());

    // TODO list set hash type
}
