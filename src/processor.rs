mod index_stats;
mod lookup;

use crate::data::{
    DataStream, DataStreams, IndexSettings, IndicesSettings, IndicesStats, Node, Nodes, ShardDoc,
};
use crate::receiver::Receiver;
use color_eyre::eyre::Result;
use lookup::Lookup;

pub async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");
    //let indices_stats = reciever.read_indices_stats().await?;
    let indices_stats: IndicesStats = reciever.get().await?;
    log::info!("Indices stats entires: {}", indices_stats.indices.len());

    let mut indices_settings: IndicesSettings = reciever.get().await?;
    let mut indices_settings_lookup: Lookup<IndexSettings> = Lookup::new();
    indices_settings.drain().for_each(|(name, settings)| {
        let id = settings.settings.index.uuid.clone();
        indices_settings_lookup
            .add(settings.settings.index)
            .with_name(&name)
            .with_id(&id);
    });
    log::info!(
        "Indices settings lookup entires: {}",
        indices_settings_lookup.len()
    );

    let mut data_streams: DataStreams = reciever.get().await?;
    let mut data_streams_lookup: Lookup<DataStream> = Lookup::new();
    data_streams.data_streams.drain(..).for_each(|data_stream| {
        let name = data_stream.name.clone();
        data_streams_lookup.add(data_stream).with_name(&name);
    });
    log::info!("Data stream lookup entires: {}", data_streams_lookup.len());

    let mut nodes: Nodes = reciever.get().await?;
    let mut nodes_lookup: Lookup<Node> = Lookup::new();
    nodes.nodes.drain().for_each(|(id, node)| {
        let name = node.name.clone();
        nodes_lookup.add(node).with_name(&name).with_id(&id);
    });
    log::info!("Nodes lookup entires: {}", nodes_lookup.len());

    log::warn!("TODO: perform calculations");
    let shard_docs = index_stats::extract_shard_docs(indices_stats)?;
    Ok(shard_docs)
}
