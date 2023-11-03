use rus_key::db::Db;
use rus_key::func::expired::ExpiredCommand;
use rus_key::func::string::StringCommand;
use rus_key::command_factory::Command;

fn set_key(db: &mut Db) {
    let command_set = StringCommand::new("set".to_string());
    let command_set_str = "key value";
    let mut parts_set = command_set_str.split_ascii_whitespace();
    let result_set = command_set.execute(&mut parts_set, db);
    assert_eq!(result_set.unwrap(), "OK".to_string());
}

fn get_current_time() -> i64 {
    let now = chrono::Utc::now();
    let timestamp = now.timestamp_millis();
    timestamp
}

fn expired_command(db: &mut Db) {
    // set string key
    set_key(db);

    let command_expired = ExpiredCommand::new("expired".to_string());
    let command_expired_str = "key 1000";
    let mut parts_expired = command_expired_str.split_ascii_whitespace();
    let result = command_expired.execute(&mut parts_expired, db);

    assert_eq!(result.unwrap(), "OK".to_string());
}

fn expireat_command(db: &mut Db) {
    // set string key
    set_key(db);

    let command_expireat = ExpiredCommand::new("expireat".to_string());
    let expireat_time = get_current_time() / 1000 + 1000;
    let command_expireat_str = "key".to_string() + " " + &expireat_time.to_string();
    let mut parts_expireat = command_expireat_str.split_ascii_whitespace();
    let result = command_expireat.execute(&mut parts_expireat, db);

    assert_eq!(result.unwrap(), "OK".to_string());
}

fn pexpire_command(db: &mut Db) {
    // set string key
    set_key(db);

    let command_pexpire = ExpiredCommand::new("pexpire".to_string());
    let pexpire_time = get_current_time();
    let command_pexpire_str = "key".to_string() + " " + &pexpire_time.to_string();
    let mut parts_pexpire = command_pexpire_str.split_ascii_whitespace();
    let result = command_pexpire.execute(&mut parts_pexpire, db);

    assert_eq!(result.unwrap(), "OK".to_string());
}

#[test]
fn test_expired_command() {
    let mut db = Db::new();
    expired_command(&mut db);
}

#[test]
fn test_expireat_command() {
    let mut db = Db::new();
    expireat_command(&mut db);
}

#[test]
fn test_pexpireat_command() {
    let mut db = Db::new();
    pexpire_command(&mut db);
}


fn ttl_command(db: &mut Db) -> Result<String, &'static str> {
    let command_ttl = ExpiredCommand::new("ttl".to_string());
    let command_ttl_str = "key";
    let mut parts_ttl = command_ttl_str.split_ascii_whitespace();
    let result = command_ttl.execute(&mut parts_ttl, db);
    result
}

#[test]
fn test_ttl_command() {
    let mut db = Db::new();
    expired_command(&mut db);
    let ttl_result = match ttl_command(&mut db) {
        Ok(ttl_string) => match ttl_string.parse::<i64>() {
            Ok(ttl) => ttl,
            Err(_) => -1,
        },
        Err(_) => -1,
    };

    assert!(0 < ttl_result && ttl_result <= 1000);
}