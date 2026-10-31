use std::collections::HashMap;

use serde::Serialize;

use crate::parser::Command;

pub struct DbEngine {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub enum EngineError {}

#[derive(Debug, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Outcome {
    Inserted,
    Updated { previous: String },
    Found { value: String },
    Deleted { previous: String },
    NotFound,
}

impl DbEngine {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn execute_command(&mut self, command: Command) -> Result<Outcome, EngineError> {
        match command {
            Command::Insert(ins) => match self.set(ins.key, ins.val) {
                Some(old) => Ok(Outcome::Updated { previous: old }),
                None => Ok(Outcome::Inserted),
            },
            Command::Get(get) => match self.get(&get.key) {
                Some(val) => Ok(Outcome::Found { value: val.clone() }),
                None => Ok(Outcome::NotFound),
            },
            Command::Delete(del) => match self.delete(&del.key) {
                Some(old) => Ok(Outcome::Deleted { previous: old }),
                None => Ok(Outcome::NotFound),
            },
        }
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}
