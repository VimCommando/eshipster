use crate::data::{IndexDoc, IndexStats, IndicesStats, ShardDoc, ShardStats};

use color_eyre::eyre::Result;

pub fn extract_shard_docs(mut indices_stats: IndicesStats) -> Result<Vec<ShardDoc>> {
    let shard_docs: Vec<ShardDoc> = indices_stats
        .indices
        .drain()
        .flat_map(|(index, index_stats)| extract_index_stats(index, index_stats))
        .collect();

    Ok(shard_docs)
}

fn extract_index_stats(index_name: String, mut index_stats: IndexStats) -> Vec<ShardDoc> {
    let index_doc = IndexDoc {
        name: index_name.clone(),
        uuid: index_stats.uuid.clone(),
        primary_shards: index_stats.primaries.shard_stats.total_count,
    };
    index_stats
        .shards
        .drain()
        .flat_map(|(shard_number, shards_stats)| {
            extract_shard_stats(shard_number, shards_stats, index_doc.clone())
        })
        .collect()
}

fn extract_shard_stats(
    shard_number: String,
    mut shard_stats: Vec<ShardStats>,
    index_doc: IndexDoc,
) -> Vec<ShardDoc> {
    shard_stats
        .drain(..)
        .filter_map(|shard_stats| {
            let number = shard_number.parse::<u16>().ok()?;
            Some(ShardDoc::new(number, shard_stats, index_doc.clone()))
        })
        .collect()
}
