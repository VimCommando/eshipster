mod index_stats;
mod lookup;

use crate::data::{DataStreams, IndicesSettings, IndicesStats, Node, Nodes, ShardDoc};
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

    let mut shards = index_stats::extract_shard_docs(indices_stats, &lookups)?;
    log::debug!("Shards starting: {}", &shards.len());
    rebalance_shards(lookups, &mut shards)?;
    log::debug!("Shards rebalanced: {}", &shards.len());
    Ok(shards)
}

fn rebalance_shards(lookups: Lookups, shards: &mut Vec<ShardDoc>) -> Result<()> {
    log::info!("Rebalancing shards");
    let role = String::from("data_hot");
    let hot_nodes: Vec<&Node> = lookups
        .node
        .get_entries()
        .iter()
        .filter(|node| node.roles.contains(&role))
        .collect();

    shards.sort_unstable_by(|a, b| {
        a.data_stream_name()
            .cmp(&b.data_stream_name())
            .then(a.index_name().cmp(&b.index_name()))
            .then(a.shard_number().cmp(&b.shard_number()))
            .then(a.primary().cmp(&b.primary()))
    });

    shards.iter_mut().enumerate().for_each(|(i, shard)| {
        let node_name = hot_nodes[i % hot_nodes.len()].name.clone();
        shard.set_desired_node(node_name)
    });

    Ok(())
}
