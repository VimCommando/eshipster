pub mod data_stream;
pub mod index_settings;
pub mod node;

use crate::data::{DataStream, IndexSettings, Node};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub struct Lookups {
    pub index: Lookup<IndexSettings>,
    pub data_stream: Lookup<DataStream>,
    pub node: Lookup<Node>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Lookup<T> {
    entries: Vec<T>,
    by_id: HashMap<String, usize>,
    by_name: HashMap<String, usize>,
    lookup: String,
}

impl<T> Lookup<T>
where
    T: Clone + Serialize + LookupDisplay,
{
    pub fn new() -> Lookup<T> {
        Lookup {
            entries: Vec::new(),
            by_id: HashMap::new(),
            by_name: HashMap::new(),
            lookup: String::from(T::display()),
        }
    }

    // Getters

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn by_name(&self, name: &str) -> Option<&T> {
        match self.by_name.get(name) {
            Some(index) => Some(&self.entries[*index]),
            None => None,
        }
    }

    pub fn by_id(&self, id: &str) -> Option<&T> {
        match self.by_id.get(id) {
            Some(index) => Some(&self.entries[*index]),
            None => None,
        }
    }

    pub fn get_entries(&self) -> &Vec<T> {
        &self.entries
    }

    // Setters

    pub fn add(&mut self, value: T) -> &mut Self {
        self.entries.push(value);
        self
    }

    pub fn with_id(&mut self, id: &str) -> &mut Self {
        self.by_id.insert(id.to_string(), self.entries.len() - 1);
        self
    }

    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.by_name
            .insert(name.to_string(), self.entries.len() - 1);
        self
    }

    // Formatters

    pub fn to_value(&self) -> Value {
        let json = serde_json::to_string(&self).expect("Failed to convert lookup to JSON");
        serde_json::from_str(&json).expect("Failed to convert lookup JSON to Value")
    }
}

pub trait LookupDisplay {
    fn display() -> &'static str;
}

impl<T: Serialize> std::fmt::Display for Lookup<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
