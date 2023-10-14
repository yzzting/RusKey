use std::collections::{HashMap, BTreeMap};

#[derive(Debug)]
pub enum DataType {
    String(String),
    List(Vec<String>),
    Set(HashMap<String, String>),
    HashMap(HashMap<String, String>),
    BTreeMap(BTreeMap<String, String>),
}

pub struct Db {
    map: HashMap<String, DataType>,
    pub not_found_message: String,
}

impl Db {
    pub fn new() -> Db {
        Db {
            map: HashMap::new(),
            not_found_message: "Key not found".to_string(),
        }
    }

    pub fn set(&mut self, key: String, value: DataType) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&DataType> {
        self.map.get(key)
    }

    pub fn check_expired(&mut self, key: String) -> bool {
        if self.map.contains_key(&key) {
            return true;
        }
        false
    }
}