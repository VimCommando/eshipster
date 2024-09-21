mod index_stats;

use crate::data::ShardDoc;
use crate::receiver::Receiver;
use color_eyre::eyre::Result;

pub async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");
    let indices_stats = reciever.read_indices_stats().await?;
    log::warn!("TODO: perform calculations");
    let shard_docs = index_stats::extract_shard_docs(indices_stats)?;
    Ok(shard_docs)
}
