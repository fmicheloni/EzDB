use std::collections::HashMap;

use crate::parser::Command;

pub struct DbEngine {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub enum EngineError {
    KeyNotFound(String),
}

impl DbEngine {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn execute_command(&mut self, command: Command) -> Result<Option<String>, EngineError> {
        match command {
            Command::Insert(ins) => {
                self.set(ins.key, ins.val);
                Ok(None)
            }
            Command::Get(get) => match self.get(&get.key) {
                Some(val) => Ok(Some(val.clone())),
                None => {
                    let error_message = format!("{} not found", &get.key);
                    Err(EngineError::KeyNotFound(error_message))
                }
            },
            Command::Delete(del) => {
                self.delete(&del.key);
                Ok(None)
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
