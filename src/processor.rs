mod index_stats;
mod lookup;

use crate::data::{DataStreams, IndicesSettings, IndicesStats, Nodes, ShardDoc};
use crate::receiver::Receiver;
use color_eyre::eyre::Result;
use lookup::{Lookup, Lookups};

pub async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");

    let lookups = Lookups {
        data_stream: Lookup::from(reciever.get::<DataStreams>().await?),
        index: Lookup::from(reciever.get::<IndicesSettings>().await?),
        node: Lookup::from(reciever.get::<Nodes>().await?),
    };

    log::info!("Data stream lookup entires: {}", lookups.data_stream.len());
    log::info!("Indices settings lookup entires: {}", lookups.index.len());
    log::info!("Nodes lookup entires: {}", lookups.node.len());

    // env_logger outputs to stderr, so we can cleanly redirect stdout to a file for debugging
    if log::max_level() >= log::Level::Trace {
        println!("{}", lookups.data_stream.to_string());
        println!("{}", lookups.index.to_string());
        println!("{}", lookups.node.to_string());
    }

    let indices_stats: IndicesStats = reciever.get().await?;
    log::info!("Indices stats entires: {}", indices_stats.indices.len());

    let shard_docs = index_stats::extract_shard_docs(indices_stats, &lookups)?;

    log::warn!("TODO: perform calculations");
    Ok(shard_docs)
}
