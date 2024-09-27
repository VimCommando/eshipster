use super::lookup::Lookups;
use crate::data::{IndexStats, IndicesStats, ShardDoc, ShardEnrich, ShardStats};
use color_eyre::eyre::Result;

pub fn extract_shard_docs(
    mut indices_stats: IndicesStats,
    lookups: &Lookups,
) -> Result<Vec<ShardDoc>> {
    let shard_docs: Vec<ShardDoc> = indices_stats
        .indices
        .drain()
        .flat_map(|(index, index_stats)| extract_index_stats(index, index_stats, lookups))
        .collect();

    Ok(shard_docs)
}

fn extract_index_stats(
    index_name: String,
    mut index_stats: IndexStats,
    lookups: &Lookups,
) -> Vec<ShardDoc> {
    let enrich = ShardEnrich {
        index: lookups.index.by_name(&index_name).cloned(),
        data_stream: lookups.data_stream.by_id(&index_name).cloned(),
        node: None,
    };
    index_stats
        .shards
        .drain()
        .flat_map(|(shard_number, shards_stats)| {
            extract_shard_stats(shard_number, shards_stats, enrich.clone(), lookups)
        })
        .collect()
}

fn extract_shard_stats(
    shard_number: String,
    mut shards_stats: Vec<ShardStats>,
    mut enrich: ShardEnrich,
    lookups: &Lookups,
) -> Vec<ShardDoc> {
    shards_stats
        .drain(..)
        .filter_map(|shard_stats| {
            let number = shard_number.parse::<u16>().ok()?;
            enrich.node = Some(
                lookups
                    .node
                    .by_id(&shard_stats.routing.node)
                    .cloned()
                    .expect("Node not found"),
            );
            Some(ShardDoc::new(number, shard_stats, enrich.clone()))
        })
        .collect()
}
