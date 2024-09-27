use super::{Lookup, LookupDisplay};
use crate::data::{DataStream, DataStreams, Indices};

impl From<DataStreams> for Lookup<DataStream> {
    fn from(mut data_streams: DataStreams) -> Self {
        let mut lookup = Lookup::<DataStream>::new();
        data_streams
            .data_streams
            .drain(..)
            .for_each(|mut data_stream| {
                let name = data_stream.name.clone();
                let indices: Indices = data_stream.indices.drain(..).collect();
                lookup.add(data_stream).with_name(&name);
                // Each data stream can have multiple indices
                indices.iter().for_each(|index| {
                    lookup.with_id(&index.index_name.clone());
                });
            });
        lookup
    }
}

impl LookupDisplay for DataStream {
    fn display() -> &'static str {
        "data_stream"
    }
}

impl std::fmt::Display for DataStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
