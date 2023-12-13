use std::collections::BTreeMap;
use std::fs;

use crate::args::Opt;

pub struct Store {
    pub url: String,
}

pub fn init() -> BTreeMap<String, String> {
    // fs read config file ./ruskey.conf
    let content = match fs::read_to_string("./ruskey.conf") {
        Ok(content) => content,
        Err(error) => {
            println!("read config file ./ruskey.conf failed: {}", error);
            return BTreeMap::new();
        }
    };

    let mut config = BTreeMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            println!("invalid config line: {}", line);
            continue;
        }

        let key = parts[0].to_string();
        let value = parts[1].to_string();
        config.insert(key, value);
    }

    config
}

pub struct Config {
    opt: Opt,
    config: BTreeMap<String, String>,
}

impl Config {
    pub fn new(opt: Opt, config: BTreeMap<String, String>) -> Self {
        Self { opt, config }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.opt
            .get(key)
            .cloned()
            .or_else(|| self.config.get(key).cloned())
    }
}
