mod index_stats;
mod lookup;

use crate::data::{DataStreams, IndicesSettings, IndicesStats, ShardDoc};
use crate::receiver::Receiver;
use color_eyre::eyre::Result;

pub async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");
    //let indices_stats = reciever.read_indices_stats().await?;
    let indices_stats: IndicesStats = reciever.get().await?;
    let indices_settings: IndicesSettings = reciever.get().await?;
    let data_streams: DataStreams = reciever.get().await?;
    log::warn!("TODO: perform calculations");
    let shard_docs = index_stats::extract_shard_docs(indices_stats)?;
    Ok(shard_docs)
}
