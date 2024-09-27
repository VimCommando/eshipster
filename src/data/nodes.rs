use super::ElasticsearchApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct Node {
    //aggregations: Value,
    attributes: Value,
    build_flavor: String,
    build_hash: String,
    build_type: String,
    component_version: Option<ComponentVersion>,
    host: String,
    //http: Value,
    index_version: Option<i64>,
    //ingest: Value,
    ip: String,
    //jvm: Value,
    //modules: Value,
    pub name: String,
    os: Value,
    //plugins: Value,
    //process: Value,
    roles: Vec<String>,
    //settings: Value,
    //thread_pool: Value,
    //total_indexing_buffer: Value,
    //transport: Value,
    //transport_address: String,
    //transport_version: Option<i64>,
    version: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct ComponentVersion {
    ml_config_version: i64,
    transform_config_version: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Nodes {
    //_nodes: Value,
    pub cluster_name: Option<String>,
    pub nodes: HashMap<String, Node>,
}

impl ElasticsearchApi for Nodes {
    fn url_path() -> String {
        "_nodes".to_string()
    }
    fn file_name() -> String {
        "nodes.json".to_string()
    }
}
