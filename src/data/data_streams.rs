use super::ElasticsearchApi;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct DataStreams {
    pub data_streams: Vec<DataStream>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DataStream {
    pub allow_custom_routing: Option<bool>,
    pub generation: u64,
    pub hidden: Option<bool>,
    pub ilm_policy: Option<String>,
    pub indices: Vec<Index>,
    pub name: String,
    pub next_generation_managed_by: Option<String>,
    pub prefer_ilm: Option<bool>,
    pub replicated: Option<bool>,
    pub rollover_on_write: Option<bool>,
    pub status: String,
    pub system: Option<bool>,
    pub template: String,
    pub timestamp_field: TimestampField,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TimestampField {
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Index {
    pub index_name: String,
    pub index_uuid: String,
    pub prefer_ilm: Option<bool>,
    pub ilm_policy: Option<String>,
    pub managed_by: Option<String>,
}

impl ElasticsearchApi for DataStreams {
    fn url_path() -> String {
        "_data_stream".to_string()
    }
    fn file_name() -> String {
        "commecrial/data_stream.json".to_string()
    }
}
