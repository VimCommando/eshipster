mod index_stats;
mod lookup;

use crate::data::{
    DataStream, DataStreams, IndexSettings, IndicesSettings, IndicesStats, Node, Nodes, ShardDoc,
};
use crate::receiver::Receiver;
use color_eyre::eyre::Result;
use lookup::Lookup;

struct Lookups {
    pub index: Lookup<IndexSettings>,
    pub data_stream: Lookup<DataStream>,
    pub node: Lookup<Node>,
}

pub async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");

    let lookup = Lookups {
        data_stream: Lookup::from(reciever.get::<DataStreams>().await?),
        index: Lookup::from(reciever.get::<IndicesSettings>().await?),
        node: Lookup::from(reciever.get::<Nodes>().await?),
    };

    log::info!("Data stream lookup entires: {}", lookup.data_stream.len());
    log::info!("Indices settings lookup entires: {}", lookup.index.len());
    log::info!("Nodes lookup entires: {}", lookup.node.len());

    let indices_stats: IndicesStats = reciever.get().await?;
    log::info!("Indices stats entires: {}", indices_stats.indices.len());

    log::warn!("TODO: perform calculations");
    let shard_docs = index_stats::extract_shard_docs(indices_stats)?;
    Ok(shard_docs)
}
