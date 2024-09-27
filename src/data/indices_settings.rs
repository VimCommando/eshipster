use super::ElasticsearchApi;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct IndexSettings {
    allocation: Option<Value>,
    auto_expand_replicas: Option<String>,
    blocks: Option<Value>,
    #[serde(default = "default_codec")]
    codec: String,
    #[serde(deserialize_with = "number_from_string")]
    creation_date: Option<i64>,
    default_pipeline: Option<String>,
    final_pipeline: Option<String>,
    hidden: Option<String>,
    lifecycle: Option<Value>,
    mapping: Option<Value>,
    #[serde(deserialize_with = "number_from_string")]
    number_of_replicas: Option<i64>,
    #[serde(deserialize_with = "number_from_string")]
    number_of_shards: Option<i64>,
    priority: Option<String>,
    provided_name: String,
    query: Option<Value>,
    #[serde(default = "default_refresh_interval")]
    refresh_interval: String,
    routing: Option<Value>,
    shard: Option<Value>,
    shard_limit: Option<Value>,
    store: Option<Value>,
    sort: Option<Value>,
    pub uuid: String,
    version: Value,
}

fn default_codec() -> String {
    String::from("default")
}

fn default_refresh_interval() -> String {
    String::from("default")
}

// Deserializing data structures

#[derive(Clone, Deserialize, Serialize)]
pub struct Settings {
    pub settings: Index,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Index {
    pub index: IndexSettings,
}

// The standard deserializer from serde_json does not deserializing numbers from
// strings. Unfortunately the _settings API frequently wraps numbers in quotes.

fn number_from_string<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::Number(num) => Ok(num.as_i64()),
        Value::String(s) => Ok(s.parse::<i64>().ok()),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom(
            "expected a number or a string representing a number",
        )),
    }
}

pub type IndicesSettings = HashMap<String, Settings>;

impl ElasticsearchApi for IndicesSettings {
    fn url_path() -> String {
        "_settings".to_string()
    }
    fn file_name() -> String {
        "settings.json".to_string()
    }
}
