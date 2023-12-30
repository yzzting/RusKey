use rus_key_command_lib::get_parts;
use rus_key_db::db::DataType;
use rus_key_db::db::Db;
use std::collections::VecDeque;
use std::str::SplitAsciiWhitespace;

pub fn list_push(parts: &mut SplitAsciiWhitespace, db: &mut Db) -> String {
    let (key, _) = get_parts(parts, false);
    let mut value_vec: VecDeque<String> = VecDeque::new();
    while let Some(value) = parts.next() {
        value_vec.push_front(value.to_string());
    }

    let mut list = match db.get(&key) {
        Some(list) => match list {
            DataType::List(list) => list.clone(),
            _ => return "wrong type".to_string(),
        },
        None => VecDeque::new(),
    };

    for value in value_vec {
        list.push_front(value);
    }
    println!("{:?}", list);
    let len = list.len();

    db.set(key.to_string(), DataType::List(list));

    len.to_string()
}
