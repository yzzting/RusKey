use std::collections::{HashMap, BTreeMap};

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    // List(Vec<String>),
    // Set(HashMap<String, String>),
    HashMap(HashMap<String, String>),
    ZSet(BTreeMap<String, String>),
}

pub trait Db: Send + Sync {
    // fn new() -> Self;
    fn set(&mut self, key: String, value: DataType);
    fn get(&self, key: &str) -> Option<&DataType>;
    fn delete(&mut self, key: &str) -> bool;
    fn check_expired(&mut self, key: &str) -> bool;
    fn randomkey(&mut self) -> Option<String>;
}
