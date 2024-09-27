use super::{Lookup, LookupDisplay};
use crate::data::DataStream;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct DataStreamDoc {
    allow_custom_routing: Option<bool>,
    generation: u64,
    hidden: Option<bool>,
    ilm_policy: Option<String>,
    is_write_index: Option<bool>,
    name: String,
    next_generation_managed_by: Option<String>,
    prefer_ilm: Option<bool>,
    replicated: Option<bool>,
    rollover_on_write: Option<bool>,
    status: String,
    system: Option<bool>,
    template: String,
    timestamp_field: String,
}

impl DataStreamDoc {
    pub fn is_write_index(&self) -> bool {
        match self.is_write_index {
            Some(value) => value,
            None => false,
        }
    }

    pub fn set_write_index(&mut self, value: bool) {
        self.is_write_index = Some(value);
    }
}

impl From<&DataStream> for DataStreamDoc {
    fn from(data_stream: &DataStream) -> Self {
        Self {
            allow_custom_routing: data_stream.allow_custom_routing,
            generation: data_stream.generation,
            hidden: data_stream.hidden,
            ilm_policy: data_stream.ilm_policy.clone(),
            is_write_index: None,
            name: data_stream.name.clone(),
            next_generation_managed_by: data_stream.next_generation_managed_by.clone(),
            prefer_ilm: data_stream.prefer_ilm,
            replicated: data_stream.replicated,
            rollover_on_write: data_stream.rollover_on_write,
            status: data_stream.status.clone(),
            system: data_stream.system.clone(),
            template: data_stream.template.clone(),
            timestamp_field: data_stream.timestamp_field.name.clone(),
        }
    }
}

impl LookupDisplay for DataStreamDoc {
    fn display() -> &'static str {
        "data_stream_doc"
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DataStreamWrapper {
    data_streams: Vec<DataStream>,
}

impl From<&String> for Lookup<DataStreamDoc> {
    fn from(string: &String) -> Self {
        let data_streams: DataStreamWrapper =
            serde_json::from_str(&string).expect("Failed to parse DataStreamData");
        let mut lookup_data_stream: Lookup<DataStreamDoc> = Lookup::new();

        for data_stream in data_streams.data_streams {
            let mut data_stream_doc = DataStreamDoc::from(&data_stream);
            let index_count = data_stream.indices.len() - 1;
            let indices: Vec<_> = data_stream.indices.into_iter().enumerate().collect();

            for (i, index) in indices {
                data_stream_doc.set_write_index(i == index_count);
                lookup_data_stream
                    .add(data_stream_doc.clone())
                    .with_id(&index.index_uuid)
                    .with_name(&index.index_name);
            }
        }
        log::debug!(
            "lookup_data_stream entries: {}",
            lookup_data_stream.entries.len(),
        );
        lookup_data_stream
    }
}
