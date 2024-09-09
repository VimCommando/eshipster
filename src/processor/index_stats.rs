use color_eyre::eyre::Result;

use crate::data::{IndicesStats, ShardDoc};

pub fn extract_shard_docs(indices_stats: IndicesStats) -> Result<Vec<ShardDoc>> {
    let mut shard_docs = Vec::new();
    for (index, index_stats) in indices_stats.indices {
        for (shard_number, shards_stats) in index_stats.shards {
            let number = shard_number
                .parse::<u16>()
                .expect("Failed to parse shard number");
            for shard_stats in shards_stats {
                let shard_doc =
                    ShardDoc::new(shard_stats, index.clone(), index_stats.uuid.clone(), number);
                shard_docs.push(shard_doc);
            }
        }
    }
    Ok(shard_docs)
}
