use std::collections::HashMap;

use crate::parser::Command;

pub struct DbEngine {
    map: HashMap<String, String>,
}

impl DbEngine {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn execute_command(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Insert(ins) => {
                self.set(ins.key, ins.val);
                Some("OK".to_string())
            }
            Command::Get(get) => match self.get(&get.key) {
                Some(val) => Some(val.clone()),
                None => None,
            },
            Command::Delete(del) => {
                self.delete(&del.key);
                Some("OK".to_string())
            }
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }
}
