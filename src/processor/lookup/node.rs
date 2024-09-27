use crate::data::Node;

use super::LookupDisplay;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Serialize)]
pub struct NodeData {
    pub attributes: Value,
    pub host: String,
    pub id: String,
    pub ip: String,
    pub name: String,
    pub os: Value,
    pub role: String,
    pub roles: Vec<String>,
    pub version: String,
}

impl NodeData {
    pub fn rename(self, name: &String) -> Self {
        NodeData {
            name: name.clone(),
            ..self
        }
    }

    pub fn with_role(self, role: &String) -> Self {
        NodeData {
            role: role.clone(),
            ..self
        }
    }
}

impl LookupDisplay for NodeData {
    fn display() -> &'static str {
        "node_data"
    }
}

impl LookupDisplay for Node {
    fn display() -> &'static str {
        "nodes"
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
