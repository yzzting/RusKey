use std::collections::HashMap;

pub struct Db {
    map: HashMap<String, String>,
    pub not_found_message: String,
}

impl Db {
    pub fn new() -> Db {
        Db {
            map: HashMap::new(),
            not_found_message: "Key not found".to_string(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}