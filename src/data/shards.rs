use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct IndicesStats {
    // _shards: Value,
    // _all: Value,
    pub indices: HashMap<String, IndexStats>,
}

#[derive(Deserialize, Serialize)]
pub struct IndexStats {
    // health: Option<String>,
    // primaries: Value,
    // total: Value,
    pub shards: HashMap<String, Vec<ShardStats>>,
    pub uuid: String,
}

#[derive(Deserialize, Serialize)]
pub struct ShardStats {
    docs: DocStats,
    indexing: IndexingStats,
    search: SearchStats,
    #[serde(skip_serializing)]
    routing: ShardRouting,
}

#[derive(Deserialize, Serialize)]
pub struct DocStats {
    count: u64,
    deleted: u64,
    total_size_in_bytes: u64,
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
    node: String,
    primary: bool,
    relocating_node: Option<String>,
    state: String,
}

#[derive(Serialize)]
pub struct ShardDoc {
    index: IndexDoc,
    shard: ShardData,
    stats: ShardStats,
}

#[derive(Serialize)]
pub struct ShardData {
    number: u16,
    #[serde(flatten)]
    routing: ShardRouting,
}

#[derive(Serialize)]
pub struct IndexDoc {
    name: String,
    uuid: String,
}

impl ShardDoc {
    pub fn new(stats: ShardStats, name: String, uuid: String, number: u16) -> Self {
        ShardDoc {
            index: IndexDoc { name, uuid },
            shard: ShardData {
                number,
                routing: stats.routing.clone(),
            },
            stats,
        }
    }
}
