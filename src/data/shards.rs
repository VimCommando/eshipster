use super::{ElasticsearchApi, IndexSettings, Node};
use crate::config;
use elasticsearch::params::Order;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct IndicesStats {
    // _shards: Value,
    // _all: Value,
    pub indices: HashMap<String, IndexStats>,
}

#[derive(Deserialize)]
pub struct IndexStats {
    // health: Option<String>,
    pub primaries: PrimaryStats,
    // total: Value,
    pub shards: HashMap<String, Vec<ShardStats>>,
    pub uuid: String,
}

#[derive(Deserialize)]
pub struct PrimaryStats {
    pub shard_stats: PrimaryShardStats,
}

#[derive(Deserialize)]
pub struct PrimaryShardStats {
    pub total_count: u16,
}

#[derive(Deserialize, Serialize)]
pub struct ShardStats {
    docs: DocStats,
    indexing: IndexingStats,
    search: SearchStats,
    #[serde(skip_serializing)]
    pub routing: ShardRouting,
}

#[derive(Deserialize, Serialize)]
pub struct DocStats {
    count: u64,
    deleted: u64,
    total_size_in_bytes: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct IndexingStats {
    index_total: u64,
    index_time_in_millis: u64,
    index_current: u64,
    index_failed: u64,
    delete_total: u64,
    delete_time_in_millis: u64,
    delete_current: u64,
    noop_update_total: u64,
    is_throttled: bool,
    throttle_time_in_millis: u64,
    write_load: f64,
}

#[derive(Deserialize, Serialize)]
pub struct SearchStats {
    open_contexts: u64,
    query_total: u64,
    query_time_in_millis: u64,
    query_current: u64,
    fetch_total: u64,
    fetch_time_in_millis: u64,
    fetch_current: u64,
    scroll_total: u64,
    scroll_time_in_millis: u64,
    scroll_current: u64,
    suggest_total: u64,
    suggest_time_in_millis: u64,
    suggest_current: u64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ShardRouting {
    #[serde(skip_serializing)]
    pub node: String,
    primary: bool,
    relocating_node: Option<String>,
    state: String,
}

#[derive(Serialize)]
struct DataStreamName {
    r#type: &'static str,
    dataset: &'static str,
    namespace: &'static str,
}

#[derive(Serialize)]
pub struct ShardDoc {
    data_stream: DataStreamName,
    #[serde(flatten)]
    enrich: ShardEnrich,
    shard: ShardData,
    stats: ShardStats,
    #[serde(rename = "@timestamp")]
    timestamp: i64,
}

impl ShardDoc {
    pub fn data_stream_name(&self) -> String {
        self.enrich
            .index
            .as_ref()
            .and_then(|i| i.data_stream.as_ref())
            .map(|d| d.name.clone())
            .unwrap_or_default()
    }

    pub fn index_name(&self) -> String {
        self.enrich
            .index
            .as_ref()
            .and_then(|i| i.name.clone())
            .unwrap_or_default()
    }

    pub fn shard_number(&self) -> u16 {
        self.shard.number
    }

    pub fn primary(&self) -> bool {
        self.shard.routing.primary
    }

    pub fn set_desired_node(&mut self, name: String) {
        self.enrich.node.as_mut().map(|n| n.desired = Some(name));
    }
}

impl std::cmp::PartialEq for ShardDoc {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl std::cmp::PartialOrd for ShardDoc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }

    fn ge(&self, other: &Self) -> bool {
        true
    }

    fn le(&self, other: &Self) -> bool {
        true
    }

    fn lt(&self, other: &Self) -> bool {
        true
    }
}

impl std::cmp::Eq for ShardDoc {}

impl std::cmp::Ord for ShardDoc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

#[derive(Serialize)]
pub struct ShardData {
    number: u16,
    #[serde(flatten)]
    pub routing: ShardRouting,
}

#[derive(Clone, Serialize)]
pub struct ShardEnrich {
    pub index: Option<IndexSettings>,
    pub node: Option<Node>,
}

impl ShardDoc {
    pub fn new(number: u16, stats: ShardStats, enrich: ShardEnrich) -> Self {
        ShardDoc {
            data_stream: DataStreamName {
                r#type: "metrics",
                dataset: "shard",
                namespace: "eshipster",
            },
            enrich,
            shard: ShardData {
                number,
                routing: stats.routing.clone(),
            },
            stats,
            timestamp: *config::START_TIME,
        }
    }

    pub fn as_value(&self) -> serde_json::Value {
        match serde_json::to_value(self) {
            Ok(value) => value,
            Err(e) => {
                log::error!("Failed to serialize ShardDoc: {}", e);
                serde_json::Value::Null
            }
        }
    }
}

impl ElasticsearchApi for IndicesStats {
    fn url_path() -> String {
        "_all/_stats?level=shards".to_string()
    }
    fn file_name() -> String {
        "indices_stats.json".to_string()
    }
}
